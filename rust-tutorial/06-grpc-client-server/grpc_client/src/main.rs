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
