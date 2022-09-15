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

use endpoint::endpoint::Endpoint;
use std::collections::HashMap;
use crate::RouteInput;
use std::io::Error;
use crate::route::RouteStrategy;
use crate::Route;
use crate::config::Sharding;
use mysql_parser::ast::*;
use crate::sharding_rewrite::ShardingRewrite;
// use proxy::proxy::MySQLNode;
use crate::sharding_rewrite::ShardingRewriteOutput;

pub trait ShardingRoute {
    type Route;

    fn sharding_dispatch(
        &mut self,
        input: &RouteInput,
        route: Self::Route
    ) -> Result<HashMap::<String, Endpoint>, Error>;
}

pub struct ShardingRouteStrategy {
    pub sharding_rewrite: ShardingRewrite,
    pub sharding_config: Vec<Sharding>,
}

impl ShardingRouteStrategy {
    pub fn build(sharding_config: Vec<Sharding>, endpoints: Vec<Endpoint>) -> ShardingRouteStrategy {
        let sharding_rewrite = ShardingRewrite::new(sharding_config.clone(), endpoints);

        ShardingRouteStrategy {
            sharding_rewrite,
            sharding_config
        }
    }

    pub fn build_plan(&mut self, sql: String, mut stmt: SqlStmt) -> Result<Vec<ShardingRewriteOutput>, Box<dyn std::error::Error>> {
        self.sharding_rewrite.set_raw_sql(sql.to_string());
        let meta = self.sharding_rewrite.get_meta(&mut stmt);
        println!("{:#?}", meta);
        self.sharding_rewrite.database_strategy(meta)
    }
}

// impl<'a> ShardingRoute for ShardingRouteStrategy<'a> {
//     type Route = Option<RouteStrategy>;

//     fn sharding_dispatch(
//         &mut self,
//         input: &RouteInput,
//         route: Self::Route
//     ) -> Result<HashMap::<String, Endpoint>, Error> {
//         let res = HashMap::new();
//         match route {
//             Some(mut r) => {
//                 let (ep, role) = r.dispatch(input).unwrap();
//                 println!("{:#?}:{:#?} -> {:#?}", ep.clone().as_ref().unwrap().addr, ep.unwrap().user, role);
//                 Ok(res)
//             }
//             None => {
//                 Ok(res)
//             }
//         }
//     }
// }