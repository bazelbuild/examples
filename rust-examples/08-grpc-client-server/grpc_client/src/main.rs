// Copyright 2024 The Bazel examples and tutorials Authors & Contributors. // All rights reserved.
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

use proto_bindings::proto::greeter_client::GreeterClient;
use proto_bindings::proto::HelloRequest;

// https://github.com/hyperium/tonic/blob/master/examples/src/helloworld/client.rs
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreeterClient::connect("http://[::1]:5042")
        .await
        .expect("[Client]: Failed to connect to server.");

    let request = tonic::Request::new(HelloRequest {
        name: "Hello gRPC".into(),
    });

    let response = client
        .say_hello(request)
        .await
        .expect("[Client]: Failed to get a response from the server");

    println!("RESPONSE={:?}", response);

    Ok(())
}
