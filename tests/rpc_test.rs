use ibc_proto::cosmos::staking::v1beta1::query_client::QueryClient;
use ibc_proto::cosmos::staking::v1beta1::QueryParamsRequest;
/// Integration tests for tendermock gRPC server.
use std::process::{Command, Stdio};
use tokio;
use tonic;

const EXECUTABLE: &str = "tendermock";
const GRPC_ADDR: &str = "http://[::1]:50051";
const JSON_RPC_ADDR: &str = "127.0.0.1:26657";
const JRPC_QUERIES: &[&str] = &[
    "abci_info.json",
    "abci_query.json",
    "block.json",
    "commit.json",
    "genesis.json",
    "status.json",
    "validators.json",
];

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
async fn rpc() {
    let _server = Server::new();
    test_grpc().await;
    for query in JRPC_QUERIES {
        test_json_rpg(query);
    }
}

async fn test_grpc() {
    let mut client = QueryClient::connect(GRPC_ADDR).await.unwrap();
    let request = tonic::Request::new(QueryParamsRequest {});
    client
        .params(request)
        .await
        .expect("gRPC 'param' request failed");
}

fn test_json_rpg(query: &str) {
    let json_response = Command::new("curl")
        .arg("-s")
        .arg("-X")
        .arg("POST")
        .arg("-H")
        .arg("Content-Type: application/json")
        .arg("-d")
        .arg(&format!("@queries/{}", query))
        .arg(JSON_RPC_ADDR)
        .stdout(Stdio::piped())
        .spawn()
        .expect("HTTP request failed")
        .stdout
        .unwrap();
    let is_success = Command::new("jq")
        .arg(".result != null and .error == null")
        .stdin(json_response)
        .output()
        .expect("Failed to parse response");
    assert_eq!(String::from_utf8_lossy(&is_success.stdout), "true\n");
}
