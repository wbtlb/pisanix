// Copyright 2022 SphereEx Authors
// 
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
// 
//     http://www.apache.org/licenses/LICENSE-2.0
// 
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::server::PisaMySQLService;
use strategy::route::RouteInput;
use pisa_error::error::Error;
use conn_pool::PoolConn;
use mysql_protocol::client::conn::ClientConn;
use crate::mysql::ReqContext;
use tracing::error;
use mysql_parser::ast::*;
use crate::{
    transaction_fsm::{TransEventName, TransFsm},
};
use mysql_protocol::server::codec::CommonPacket;
use mysql_protocol::err::ProtocolError;
use bytes::BytesMut;
use tokio_util::codec::Encoder;
use mysql_protocol::server::codec::PacketSend;
use tokio_util::codec::Decoder;
use tokio::io::{AsyncRead, AsyncWrite};
use pisa_error::error::ErrorKind;
use mysql_parser::ast::SelectStmt;
use futures::SinkExt;
use bytes::BufMut;
use mysql_protocol::server::codec::make_eof_packet;
use futures::StreamExt;
use mysql_protocol::row::RowData;
use mysql_protocol::row::RowDataTyp;
use mysql_protocol::util::BufExt;
// use parking_lot::Mutex;
use std::sync::Arc;
use tokio::sync::Mutex;
use futures::stream::FuturesUnordered;
use std::thread;

#[derive(Debug, Clone)]
pub struct RewriteOutput {
    // change: Change,
    sql: String,
    endpoint: String,
}

fn rewrite_mock() -> Vec<RewriteOutput> {
    vec![
        RewriteOutput {
            sql: "select * from mixer.test_shard_hash_0000 order by id limit 2".to_string(),
            endpoint: "127.0.0.1:3306".to_string(),
        },
        RewriteOutput {
           sql: "select * from mixer.test_shard_hash_0001 order by id limit 2".to_string(),
           endpoint: "127.0.0.1:3306".to_string(),
        },
        RewriteOutput {
           sql: "select * from mixer.test_shard_hash_0002 order by id".to_string(),
           endpoint: "127.0.0.1:3306".to_string(),
        },
        RewriteOutput {
           sql: "select * from mixer.test_shard_hash_0003 order by id".to_string(),
           endpoint: "127.0.0.1:3306".to_string(),
        },
    ]
}

impl<T, C> PisaMySQLService<T, C>
where
    T: AsyncRead + AsyncWrite + Unpin + Send,
    C: Decoder<Item = BytesMut>
        + Encoder<PacketSend<Box<[u8]>>, Error = ProtocolError>
        + Send
        + CommonPacket,
{
    pub async fn query_inner_get_conn(
        req: &mut ReqContext<T, C>,
        sql: &str,
        payload: &[u8],
    ) -> Result<PoolConn<ClientConn>, Error> {
        match Self::get_ast(req, sql) {
            Err(err) => {
                error!("err: {:?}", err);
                Self::fsm_trigger(
                    &mut req.fsm,
                    TransEventName::QueryEvent,
                    RouteInput::Statement(sql),
                )
                .await
            }

            Ok(stmt) => match &stmt[0] {
                SqlStmt::Set(stmt) => {
                    Self::handle_set_stmt(req, stmt, sql).await;
                    req.fsm.get_conn().await
                }
                //TODO: split sql stmt for sql audit
                SqlStmt::BeginStmt(_stmt) => {
                    Self::fsm_trigger(
                        &mut req.fsm,
                        TransEventName::StartEvent,
                        RouteInput::Transaction(sql),
                    )
                    .await
                }

                SqlStmt::Start(_stmt) => {
                    Self::fsm_trigger(
                        &mut req.fsm,
                        TransEventName::StartEvent,
                        RouteInput::Transaction(sql),
                    )
                    .await
                }

                SqlStmt::Commit(_stmt) => {
                    Self::fsm_trigger(
                        &mut req.fsm,
                        TransEventName::CommitRollBackEvent,
                        RouteInput::Transaction(sql),
                    )
                    .await
                }

                SqlStmt::Rollback(_stmt) => {
                    Self::fsm_trigger(
                        &mut req.fsm,
                        TransEventName::CommitRollBackEvent,
                        RouteInput::Transaction(sql),
                    )
                    .await
                }

                SqlStmt::SelectStmt(stmt) => {
                    Ok(Self::handle_select(stmt, sql, req, payload).await.unwrap())
                }

                SqlStmt::InsertStmt(_stmt) => {
                    Self::handle_insert(sql, req).await
                }

                SqlStmt::UpdateStmt(_stmt) => {
                    Self::handle_update(sql, req).await
                }

                SqlStmt::DeleteStmt(_stmt) => {
                    Self::handle_delete(sql, req).await
                }

                _ => {
                    Ok(Self::handle_other(sql, req, payload).await.unwrap())
                }
            },
        }
    }

    async fn handle_select(stmt: &SelectStmt, sql: &str, req: &mut ReqContext<T, C>, payload: &[u8]) -> Result<PoolConn<ClientConn>, Error> {
        let mut client_conn = Self::fsm_trigger(
            &mut req.fsm,
            TransEventName::QueryEvent,
            RouteInput::Statement(sql),
        )
        .await.unwrap();
        
        // build rewrite input structure
        let plan = req.sharding.clone().lock().build_plan(sql.to_string(), mysql_parser::ast::SqlStmt::SelectStmt(stmt.clone()), "t_order".to_string());

        // rewrite sql
        let rewrite_output = rewrite_mock();

        let mut buf: BytesMut = BytesMut::new(); 
        let mut header = BytesMut::new();
        let mut row = BytesMut::new();

        let mut exec = FuturesUnordered::new();
        //let mut conns = vec![];

        for ro in rewrite_output.clone() {
             let client_conn = Self::fsm_trigger(
                 &mut req.fsm,
                 TransEventName::QueryEvent,
                RouteInput::Statement(sql),
             );

             exec.push(client_conn);
        //     // let payload = ro.sql.as_bytes();
        //     //exec.push(client_conn.send_query(ro.sql.as_bytes()));
        //     conns.push(client_conn);
        }
        

        //let conns = conns.into_boxed_slice();
        //for (idx, v) in conns.iter_mut().enumerate() {
        //    exec.push(v.send_query(rewrite_output[idx].sql.as_bytes()));
        //}
        // for ro in rewrite_output {
        //     let payload = ro.sql.as_bytes();
        //     let stream = match client_conn.send_query(payload).await {
        //         Ok(stream) => stream,
        //         Err(err) => return Err(Error::new(ErrorKind::Protocol(err))),
        //     };
           
        //     // let mut res = client_conn.query_result(ro.sql.as_bytes()).await.unwrap().unwrap();
        //     // let hh = Self::handle_resultset_header(req, stream).await.unwrap();
        //     //while let Some(data) = res.next().await {
        //     //    let mut row = data.unwrap();
        //     //    let strr = row.decode_with_name::<String>("str").unwrap();
        //     //    //match row {
        //     //    //    RowDataTyp::Text(inner_row_text) => {
        //     //    //        //buf.put_slice(&inner_row_text.buf);
        //     //    //        println!("{:?}", inner_row_text.buf);
        //     //    //    },
        //     //    //    RowDataTyp::Binary(inner_row_binary) => println!("{:?}", inner_row_binary.buf),
        //     //    //}
        //     //}

        //     (header, row) = Self::handle_query_resultset1(req, stream).await.unwrap();
        //     buf.put_slice(&row);
        // }

        header.put_slice(&buf);
        let _ = req
             .framed
             .codec_mut()
             .encode(PacketSend::EncodeOffset(make_eof_packet()[4..].into(), header.len()), &mut header);
       
        req.framed.send(PacketSend::Origin(header[..].into())).await.unwrap();

        Ok(client_conn)
        // Ok(())
    }

    async fn handle_insert(sql: &str, req: &mut ReqContext<T, C>) -> Result<PoolConn<ClientConn>, Error> {
        Self::fsm_trigger(
            &mut req.fsm,
            TransEventName::QueryEvent,
            RouteInput::Statement(sql),
        ).await
    }

    async fn handle_update(sql: &str, req: &mut ReqContext<T, C>) -> Result<PoolConn<ClientConn>, Error> {
        Self::fsm_trigger(
            &mut req.fsm,
            TransEventName::QueryEvent,
            RouteInput::Statement(sql),
        ).await
    }

    async fn handle_delete(sql: &str, req: &mut ReqContext<T, C>) -> Result<PoolConn<ClientConn>, Error> {
        Self::fsm_trigger(
            &mut req.fsm,
            TransEventName::QueryEvent,
            RouteInput::Statement(sql),
        ).await
    }

    async fn handle_other(sql: &str, req: &mut ReqContext<T, C>, payload: &[u8]) -> Result<PoolConn<ClientConn>, Error> {
        let mut client_conn = Self::fsm_trigger(
            &mut req.fsm,
            TransEventName::QueryEvent,
            RouteInput::Statement(sql),
        )
        .await.unwrap();

        // query_inner:
        let stream = match client_conn.send_query(payload).await {
            Ok(stream) => stream,
            Err(err) => return Err(Error::new(ErrorKind::Protocol(err))),
        };

        // Self::handle_query_resultset(req, stream).await.map_err(ErrorKind::from)?;
        Self::handle_query_resultset(req, stream).await.unwrap();

        Ok(client_conn)
    }
}
