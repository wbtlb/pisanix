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
use std::sync::Arc;
use tokio::sync::Mutex;
use futures::stream::FuturesOrdered;
use std::thread;
use endpoint::endpoint::Endpoint;
use mysql_protocol::client::codec::ResultsetStream;
use mysql_protocol::util::*;
use mysql_protocol::mysql_const::*;
use mysql_protocol::server::codec::ok_packet;

#[derive(Debug, Clone)]
pub struct RewriteOutput {
    // change: Change,
    sql: String,
    endpoint: String,
}

fn rewrite_mock_select() -> Vec<RewriteOutput> {
    vec![
        RewriteOutput {
            sql: "select * from mixer.test_shard_hash_0000 order by id".to_string(),
            endpoint: "127.0.0.1:3306".to_string(),
        },
        RewriteOutput {
           sql: "select * from mixer.test_shard_hash_0001 order by id".to_string(),
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

fn rewrite_mock_insert() -> Vec<RewriteOutput> {
    vec![
        RewriteOutput {
            sql: "insert into mixer.test_shard_hash_0000(id, str, f, u, i) values(20, 'aabbcc', 3.14, 255, -127)".to_string(),
            endpoint: "127.0.0.1".to_string(),
        },
        RewriteOutput {
            sql: "insert into mixer.test_shard_hash_0001(id, str, f, u, i) values(18, 'aabbcc', 3.14, 255, -127)".to_string(),
            endpoint: "127.0.0.1".to_string(),
        },
        RewriteOutput {
            sql: "insert into mixer.test_shard_hash_0002(id, str, f, u, i) values(21, 'aabbcc', 3.14, 255, -127)".to_string(),
            endpoint: "127.0.0.1".to_string(),
        },
        RewriteOutput {
            sql: "insert into mixer.test_shard_hash_0003(id, str, f, u, i) values(16, 'aabbcc', 3.14, 255, -127)".to_string(),
            endpoint: "127.0.0.1".to_string(),
        }
    ]
}

fn rewrite_mock_delete() -> Vec<RewriteOutput> {
    vec![
        RewriteOutput {
            sql: "delete from mixer.test_shard_hash_0000 where id = 20".to_string(),
            endpoint: "127.0.0.1".to_string(),
        },
        RewriteOutput {
            sql: "delete from mixer.test_shard_hash_0001 where id = 18".to_string(),
            endpoint: "127.0.0.1".to_string(),
        },
        RewriteOutput {
            sql: "delete from mixer.test_shard_hash_0002 where id = 21".to_string(),
            endpoint: "127.0.0.1".to_string(),
        },
        RewriteOutput {
            sql: "delete from mixer.test_shard_hash_0003 where id = 16".to_string(),
            endpoint: "127.0.0.1".to_string(),
        }
    ]
}

fn rewrite_mock_update() -> Vec<RewriteOutput> {
    vec![
        RewriteOutput {
            sql: "update mixer.test_shard_hash_0000 set str = 'dasheng' where id = 20".to_string(),
            endpoint: "127.0.0.1".to_string(),
        },
        RewriteOutput {
            sql: "update mixer.test_shard_hash_0001 set str = 'dasheng' where id = 18".to_string(),
            endpoint: "127.0.0.1".to_string(),
        },
        RewriteOutput {
            sql: "update mixer.test_shard_hash_0002 set str = 'dasheng' where id = 21".to_string(),
            endpoint: "127.0.0.1".to_string(),
        },
        RewriteOutput {
            sql: "update mixer.test_shard_hash_0003 set str = 'dasheng' where id = 16".to_string(),
            endpoint: "127.0.0.1".to_string(),
        }
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

                SqlStmt::SelectStmt(stmt) => Ok(Self::handle_select(stmt, sql, req, payload).await.unwrap()),

                SqlStmt::InsertStmt(stmt) => Ok(Self::handle_insert(stmt, sql, req, payload).await.unwrap()),
                
                SqlStmt::UpdateStmt(stmt) => Ok(Self::handle_update(stmt, sql, req, payload).await.unwrap()),

                SqlStmt::DeleteStmt(stmt) => Ok(Self::handle_delete(stmt, sql, req, payload).await.unwrap()),

                _ => Ok(Self::handle_other(sql, req, payload).await.unwrap()),
            },
        }
    }

    async fn take10(s:&mut ResultsetStream<'_>) -> Vec<BytesMut> {
        let mut res: Vec<BytesMut> = vec![];
        for _ in 0 .. 10 {
            res.push(s.next().await.unwrap().unwrap());
        }
        res
    }

    async fn handle_select(mut stmt: &SelectStmt, sql: &str, req: &mut ReqContext<T, C>, payload: &[u8]) -> Result<PoolConn<ClientConn>, Error> {
        let mut client_conn = Self::fsm_trigger(
            &mut req.fsm,
            TransEventName::QueryEvent,
            RouteInput::Statement(sql),
        )
        .await.unwrap();

        let rewrite_output = req.sharding.clone().lock().build_plan(sql.to_string(), mysql_parser::ast::SqlStmt::SelectStmt(stmt.clone())).unwrap();
        println!("rewrite_output: {:#?}", rewrite_output);

        // rewrite sql
        let rewrite_output = rewrite_mock_select();

        let mut exec = FuturesOrdered::new();

        for ro in rewrite_output.iter() {
            let mut pool = req.pool.clone();
            let ro = ro.clone();
            let factory = ClientConn::with_opts(
                "root".to_string(),
                "12345678".to_string(),
                "127.0.0.1:3306".to_string()
            );
            pool.set_factory(factory);
            let f = tokio::spawn(async move {
                pool.get_conn_with_endpoint(&ro.endpoint).await.unwrap()
            });

            exec.push(f);
        }

        let conns = exec.map(|x| x.unwrap()).collect::<Vec<_>>().await;

        let mut rss = vec![];
        for (mut conn, ro) in conns.into_iter().zip(rewrite_output.iter()) {
            let sql = ro.sql.clone();
            let f = async move {
                let res = conn.send_query1(sql.as_bytes()).await;
                (conn, res)
            };
            rss.push(f);
        }

        let mut res = futures::future::try_join_all(rss.into_iter().map(tokio::spawn)).await.unwrap();

        let mut exec1 = FuturesOrdered::new();

        for task in res.iter_mut() {
            let mut rs = ResultsetStream::new(task.0.framed.as_mut());
            exec1.push(async move {
                let mut nnrow = BytesMut::new();
                let mut nheader = BytesMut::new();

                let data = rs.next().await;

                let header = data.unwrap().unwrap();
                nheader.extend_from_slice(&header);
                // let ok_or_err = header[4];

                // get column count
                let (cols, ..) = length_encode_int(&header[4..]);

                // get column
                for _ in 0..cols {
                    let data = rs.next().await;
                    let data = data.unwrap().unwrap();
                    nheader.put_slice(&data);
                }
                
                // extend eof packet
                nheader.extend_from_slice(&[5, 0, 0, 0, 0xfe, 0, 0, 2, 0]);

                // skip eof
                let _ = rs.next().await;

                loop {
                    let data = rs.next().await;
                    let row = data.unwrap().unwrap();

                    if is_eof(&row) {
                        break;
                    }

                    nnrow.put_slice(&row);
                }

                (nheader, nnrow)
            });
        }

        let mut header = BytesMut::new();
        while let Some((nheader, res)) = exec1.next().await {
            if nheader[4] == ERR_HEADER {
                req.framed.send(PacketSend::Encode(nheader[4..].into())).await.unwrap();
                break;
            }

            if header.len() == 0 {
                header.put_slice(&nheader);
            }
            
            // sort();
            header.put_slice(&res);
        }

        let _ = req
             .framed
             .codec_mut()
             .encode(PacketSend::EncodeOffset(make_eof_packet()[4..].into(), header.len()), &mut header);

        req.framed.send(PacketSend::Origin(header[..].into())).await.unwrap();

        Ok(client_conn)
    }

    async fn handle_insert(stmt: &InsertStmt, sql: &str, req: &mut ReqContext<T, C>, payload: &[u8]) -> Result<PoolConn<ClientConn>, Error> {
        let mut client_conn = Self::fsm_trigger(
            &mut req.fsm,
            TransEventName::QueryEvent,
            RouteInput::Statement(sql),
        )
        .await.unwrap();
        
        let rewrite_output = req.sharding.clone().lock().build_plan(sql.to_string(), mysql_parser::ast::SqlStmt::InsertStmt(Box::new(stmt.clone()))).unwrap();
        let rewrite_output = rewrite_mock_insert();

        let mut exec = FuturesOrdered::new();

        for ro in rewrite_output.iter() {
            let mut pool = req.pool.clone();
            let ro = ro.clone();
            let factory = ClientConn::with_opts(
                "root".to_string(),
                "12345678".to_string(),
                "127.0.0.1:3306".to_string()
            );
            pool.set_factory(factory);
            let f = tokio::spawn(async move {
                pool.get_conn_with_endpoint(&ro.endpoint).await.unwrap()
            });

            exec.push(f);
        }

        let conns = exec.map(|x| x.unwrap()).collect::<Vec<_>>().await;

        let mut rss = vec![];
        for (mut conn, ro) in conns.into_iter().zip(rewrite_output.iter()) {
            let sql = ro.sql.clone();
            let f = async move {
                let res = conn.send_query1(sql.as_bytes()).await;
                (conn, res)
            };
            rss.push(f);
        }

        let mut res = futures::future::try_join_all(rss.into_iter().map(tokio::spawn)).await.unwrap();

        let mut exec1 = FuturesOrdered::new();

        for task in res.iter_mut() {
            let mut rs = ResultsetStream::new(task.0.framed.as_mut());
            exec1.push(async move {
                let data = rs.next().await;
                let header = data.unwrap().unwrap();
                header
            });
        }

        while let Some(header) = exec1.next().await {
            if !is_ok(&header) {
                req.framed.send(PacketSend::Encode(header[4..].into())).await.unwrap();
                break;
            }
        }

        req.framed.send(PacketSend::Encode(ok_packet()[4..].into())).await.unwrap();

        Ok(client_conn)
    }

    async fn handle_update(stmt: &UpdateStmt, sql: &str, req: &mut ReqContext<T, C>, payload: &[u8]) -> Result<PoolConn<ClientConn>, Error> {
        let mut client_conn = Self::fsm_trigger(
            &mut req.fsm,
            TransEventName::QueryEvent,
            RouteInput::Statement(sql),
        )
        .await.unwrap();

        // rewrite sql
        let rewrite_output = req.sharding.clone().lock().build_plan(sql.to_string(), mysql_parser::ast::SqlStmt::UpdateStmt(Box::new(stmt.clone()))).unwrap();
        let rewrite_output = rewrite_mock_update();

        let mut exec = FuturesOrdered::new();

        for ro in rewrite_output.iter() {
            let mut pool = req.pool.clone();
            let ro = ro.clone();
            let factory = ClientConn::with_opts(
                "root".to_string(),
                "12345678".to_string(),
                "127.0.0.1:3306".to_string()
            );
            pool.set_factory(factory);
            let f = tokio::spawn(async move {
                pool.get_conn_with_endpoint(&ro.endpoint).await.unwrap()
            });

            exec.push(f);
        }

        let conns = exec.map(|x| x.unwrap()).collect::<Vec<_>>().await;

        let mut rss = vec![];
        for (mut conn, ro) in conns.into_iter().zip(rewrite_output.iter()) {
            let sql = ro.sql.clone();
            let f = async move {
                let res = conn.send_query1(sql.as_bytes()).await;
                (conn, res)
            };
            rss.push(f);
        }

        let mut res = futures::future::try_join_all(rss.into_iter().map(tokio::spawn)).await.unwrap();

        let mut exec1 = FuturesOrdered::new();

        for task in res.iter_mut() {
            let mut rs = ResultsetStream::new(task.0.framed.as_mut());
            exec1.push(async move {
                let data = rs.next().await;
                let header = data.unwrap().unwrap();
                header
            });
        }

        while let Some(header) = exec1.next().await {
            if !is_ok(&header) {
                req.framed.send(PacketSend::Encode(header[4..].into())).await.unwrap();
                break;
            }
        }

        req.framed.send(PacketSend::Encode(ok_packet()[4..].into())).await.unwrap();

        Ok(client_conn)
    }

    async fn handle_delete(stmt: &DeleteStmt, sql: &str, req: &mut ReqContext<T, C>, payload: &[u8]) -> Result<PoolConn<ClientConn>, Error> {
        let mut client_conn = Self::fsm_trigger(
            &mut req.fsm,
            TransEventName::QueryEvent,
            RouteInput::Statement(sql),
        )
        .await.unwrap();

        let rewrite_output = req.sharding.clone().lock().build_plan(sql.to_string(), mysql_parser::ast::SqlStmt::DeleteStmt(Box::new(stmt.clone()))).unwrap();
        let rewrite_output = rewrite_mock_delete();

        let mut exec = FuturesOrdered::new();

        for ro in rewrite_output.iter() {
            let mut pool = req.pool.clone();
            let ro = ro.clone();
            let factory = ClientConn::with_opts(
                "root".to_string(),
                "12345678".to_string(),
                "127.0.0.1:3306".to_string()
            );
            pool.set_factory(factory);
            let f = tokio::spawn(async move {
                pool.get_conn_with_endpoint(&ro.endpoint).await.unwrap()
            });

            exec.push(f);
        }

        let conns = exec.map(|x| x.unwrap()).collect::<Vec<_>>().await;

        let mut rss = vec![];
        for (mut conn, ro) in conns.into_iter().zip(rewrite_output.iter()) {
            let sql = ro.sql.clone();
            let f = async move {
                let res = conn.send_query1(sql.as_bytes()).await;
                (conn, res)
            };
            rss.push(f);
        }

        let mut res = futures::future::try_join_all(rss.into_iter().map(tokio::spawn)).await.unwrap();

        let mut exec1 = FuturesOrdered::new();

        for task in res.iter_mut() {
            let mut rs = ResultsetStream::new(task.0.framed.as_mut());
            exec1.push(async move {
                let data = rs.next().await;
                let header = data.unwrap().unwrap();
                header
            });
        }

        while let Some(header) = exec1.next().await {
            if !is_ok(&header) {
                req.framed.send(PacketSend::Encode(header[4..].into())).await.unwrap();
                break;
            }
        }

        req.framed.send(PacketSend::Encode(ok_packet()[4..].into())).await.unwrap();

        Ok(client_conn)
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

        // let ep = client_conn.get_endpoint();
        Ok(client_conn)
        // Ok(ep)
    }
}
