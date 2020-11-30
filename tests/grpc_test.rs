/// Integration tests for tendermock gRPC server.
use std::process::Command;
use ibc_proto::cosmos::staking::v1beta1::QueryParamsRequest;
use ibc_proto::cosmos::staking::v1beta1::query_client::QueryClient;
use tonic;
use tokio;

const EXECUTABLE: &str = "tendermock";
const GRPC_ADDR: &str = "http://[::1]:50051";

#[tokio::test]
async fn test_param() {
    let mut server = tendermock_command()
        .spawn()
        .expect("Failed to start server process");
    tokio::time::delay_for(tokio::time::Duration::new(3,0)).await;
    let mut client = QueryClient::connect(GRPC_ADDR).await.unwrap();
    let request = tonic::Request::new(QueryParamsRequest {});
    let response = client.params(request).await.unwrap();
    println!("Response: {:?}", response);
    server.kill().unwrap();
}

fn tendermock_command() -> Command {
    Command::new(format!("./target/debug/{}", EXECUTABLE))
}
