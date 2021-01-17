use ibc_proto::cosmos::staking::v1beta1::query_client::QueryClient;
use ibc_proto::cosmos::staking::v1beta1::QueryParamsRequest;
/// Integration tests for tendermock JsonRPC and gRPC server.
use std::process::{Command, Stdio};
use tendermock::Tendermock;
use tokio;
use tonic;

const JSON_RPC_ADDR: &str = "127.0.0.1:26657";
const JSON_RPC_ADDR_2: &str = "127.0.0.1:26658";
const GRPC_ADDR: &str = "127.0.0.1:50051";
const GRPC_ADDR_2: &str = "127.0.0.1:50052";
const GRPC_URL: &str = "http://127.0.0.1:50051";
const JRPC_QUERIES: &[&str] = &[
    "abci_info.json",
    "abci_query.json",
    "block.json",
    "commit.json",
    "genesis.json",
    "status.json",
    "validators.json",
];

/// Spwan a server in another thread.
fn start_server() {
    let mut node = Tendermock::new();
    node.add_interface(JSON_RPC_ADDR.parse().unwrap(), GRPC_ADDR.parse().unwrap())
        .add_interface(
            JSON_RPC_ADDR_2.parse().unwrap(),
            GRPC_ADDR_2.parse().unwrap(),
        );
    std::thread::spawn(move || node.start());
    std::thread::sleep(std::time::Duration::new(2, 0));
}

#[tokio::test]
async fn rpc() {
    start_server();
    test_grpc().await;
    for query in JRPC_QUERIES {
        test_json_rpg(query, JSON_RPC_ADDR_2);
    }
    test_json_rpg(JRPC_QUERIES[0], JSON_RPC_ADDR_2)
}

async fn test_grpc() {
    let mut client = QueryClient::connect(GRPC_URL).await.unwrap();
    let request = tonic::Request::new(QueryParamsRequest {});
    client
        .params(request)
        .await
        .expect("gRPC 'param' request failed");
}

fn test_json_rpg(query: &str, jrpc_addr: &str) {
    let json_response = Command::new("curl")
        .arg("-s")
        .arg("-X")
        .arg("POST")
        .arg("-H")
        .arg("Content-Type: application/json")
        .arg("-d")
        .arg(&format!("@queries/{}", query))
        .arg(jrpc_addr)
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
