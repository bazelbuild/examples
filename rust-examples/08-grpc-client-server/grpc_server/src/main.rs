// Copyright 2024 The Bazel examples and tutorials Authors & Contributors.
// All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::error::Error;

use tonic::transport::Server;

use proto_bindings::proto::greeter_server::GreeterServer;

use crate::server::MyGreeter;

mod server;
mod shutdown_utils;

// https://github.com/hyperium/tonic/blob/master/examples/src/helloworld/server.rs
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = "[::1]:5042"
        .parse()
        .expect("[Server]: Failed to parse socket address");

    let grpc_svc = GreeterServer::new(MyGreeter::new());

    // Shutdown signal handler
    let signal = shutdown_utils::signal_handler("gRPC Greeter server");

    let grpc_server = Server::builder()
        .add_service(grpc_svc)
        .serve_with_shutdown(addr, signal);

    let grpc_handle = tokio::spawn(grpc_server);

    println!("GreeterServer listening on {}", addr);
    match tokio::try_join!(grpc_handle) {
        Ok(_) => {}
        Err(e) => {
            println!("[Server]: Error: Failed to start gRPC Greeter server.");
            println!("[Server]: Error: {:?}", e);
        }
    }

    Ok(())
}
