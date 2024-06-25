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
