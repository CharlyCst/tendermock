use ibc_proto::cosmos::staking::v1beta1::query_client::QueryClient;
use ibc_proto::cosmos::staking::v1beta1::QueryParamsRequest;
/// Integration tests for tendermock gRPC server.
use std::process::Command;
use tokio;
use tonic;

const EXECUTABLE: &str = "tendermock";
const GRPC_ADDR: &str = "http://[::1]:50051";

/// Represents a server process.
/// A server is started on instantiation, and killed when it goes out of scope.
struct Server {
    process: std::process::Child,
}

impl Server {
    pub fn new() -> Self {
        let child = Command::new(format!("./target/debug/{}", EXECUTABLE))
            .spawn()
            .expect("Failed to start server process");
        std::thread::sleep(std::time::Duration::new(2, 0));
        Server { process: child }
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        self.process.kill().unwrap();
    }
}

#[tokio::test]
async fn test_param() {
    let _server = Server::new();
    let mut client = QueryClient::connect(GRPC_ADDR).await.unwrap();
    let request = tonic::Request::new(QueryParamsRequest {});
    let response = client.params(request).await.unwrap();
    println!("Response: {:?}", response);
}
