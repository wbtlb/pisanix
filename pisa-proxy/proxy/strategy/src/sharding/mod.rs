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

pub trait ShardingRoute {
    type Route;

    fn sharding_dispatch(
        &mut self,
        input: &RouteInput,
        route: Self::Route
    ) -> Result<HashMap::<String, Endpoint>, Error>;
}

pub struct ShardingRouteStrategy {
    pub sharding_config: HashMap<String, Sharding>,
    // pub sharding_config: Vec<Sharding>,
    // pub statement: Option<SqlStmt>,
    // pub sql: String,
}

#[derive(Debug)]
pub struct Plan {
    // sharding_config: HashMap::<String, Sharding>,
    sharding_rule: Sharding,
    sql: String,
    stmt: SqlStmt
}

impl ShardingRouteStrategy {
    pub fn build(sharding_config: Vec<Sharding>) -> ShardingRouteStrategy {
        let mut sharding = HashMap::<String, Sharding>::new();

        for sc in sharding_config {
            sharding.insert(sc.clone().table_name, sc);
        }
        ShardingRouteStrategy {
            sharding_config: sharding
        }
    }

    pub fn sharding_test(&self) {
        println!("sharding ...");
    }

    pub fn build_plan(&self, sql: String, stmt: SqlStmt, table_name: String) -> Plan {
        let sharding_rule = self.sharding_config.get(&table_name).unwrap().clone();
        Plan {
            sharding_rule,
            sql,
            stmt,
        }
    }
}


impl ShardingRoute for ShardingRouteStrategy {
    type Route = Option<RouteStrategy>;

    fn sharding_dispatch(
        &mut self,
        input: &RouteInput,
        route: Self::Route
    ) -> Result<HashMap::<String, Endpoint>, Error> {
        let res = HashMap::new();
        match route {
            Some(mut r) => {
                let (ep, role) = r.dispatch(input).unwrap();
                println!("{:#?}:{:#?} -> {:#?}", ep.clone().as_ref().unwrap().addr, ep.unwrap().user, role);
                Ok(res)
            }
            None => {
                Ok(res)
            }
        }
    }
}