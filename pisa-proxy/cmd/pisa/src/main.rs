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

#![warn(unused_must_use)]
#![allow(dead_code)]

use std::str::FromStr;

use tokio::runtime::{Builder, Runtime};
use tracing::{error, info, warn, Level};
extern crate tokio;

use config::config::PisaConfig;
use proxy::factory::Factory;
use server::backend_const::{BACKEND_TYPE_MYSQL, BACKEND_TYPE_SHARDING_PROXY};

fn main() {
    let config = PisaConfig::load_config();
    tracing_subscriber::fmt()
        .with_max_level(Level::from_str(config.admin.log_level.as_str()).ok())
        .init();

    let mut servers = Vec::with_capacity(config.get_proxies().len());
    let http_server = http::http::new_rocket_server(config.clone());

    build_runtime().block_on(async move {
        for proxy_config in config.get_proxies() {
            let cfg = proxy_config.clone();
            let factory = server::server::SimpleFactory::new(cfg, config.clone());
            match proxy_config.backend_type.as_str() {
                BACKEND_TYPE_MYSQL => servers.push(tokio::spawn(server::server::new_proxy_server(
                    factory.build_proxy(proxy::factory::ProxyKind::MySQL),
                ))),
                BACKEND_TYPE_SHARDING_PROXY => {
                    servers.push(tokio::spawn(server::server::new_proxy_server(
                        factory.build_proxy(proxy::factory::ProxyKind::ShardingProxy),
                    )))
                }
                &_ => {}
            }
        }

        servers.push(tokio::spawn(http::http::bg_task(http_server)));

        for server in servers {
            if let Err(e) = server.await {
                error!("{:?}", e)
            }
        }
    });
}

/// build runtime, build Tokio runtime
pub fn build_runtime() -> Runtime {
    let num_cpus = num_cpus::get();
    match num_cpus {
        0 | 1 => {
            info!("pisa-proxy running on current thread");
            Builder::new_current_thread()
                .thread_name("pisa-proxy")
                .enable_all()
                .build()
                .expect("failed to build runtime")
        }
        num_cpus => {
            info!("pisa-proxy running on multi thread");
            Builder::new_multi_thread()
                .thread_name("pisa-proxy")
                .worker_threads(num_cpus)
                .max_blocking_threads(num_cpus)
                .enable_all()
                .build()
                .expect("failed to build runtime")
        }
    }
}