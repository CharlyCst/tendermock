use jsonrpc_core::IoHandler;
use jsonrpc_http_server::ServerBuilder;
use tokio;

mod abci;
mod avl;
mod chain;
mod cli;
mod config;
mod data;
mod grpc;
mod init;
mod json_rpc;
mod node;
mod store;
mod test_node;

use json_rpc::Rpc;

fn main() {
    let args = cli::get_args();
    let config = if let Some(config_path) = args.config {
        config::Config::load(config_path)
    } else {
        config::Config::default()
    };
    let mut node = node::Node::new(&config);
    init::init(&mut node, &config);
    let server = json_rpc::Server::new(args.verbose, node);

    // Automatically grow the chain
    let node = server.get_node();
    let block_interval = args.block;
    let verbose = args.verbose;
    let growth_node = node.clone();
    std::thread::spawn(move || schedule_growth(growth_node, block_interval, verbose));

    // Start JsonRpc server
    println!("Starting JsonRPC");
    let mut io = IoHandler::new();
    io.extend_with(server.to_delegate());
    let server = ServerBuilder::new(io)
        .start_http(
            &format!("127.0.0.1:{}", &args.port)
                .parse()
                .expect("Invalid IP address or port"),
        )
        .expect("Unable to start RPC server");

    // Start the grpc server
    println!("Starting gRPC");
    let addr = "[::1]:50051".parse().unwrap();
    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(async {
            grpc::new(node).serve(addr).await.unwrap();
        });

    server.wait();
}

/// Push a new block on the chain every `interval` seconds.
pub fn schedule_growth<S: store::Storage>(node: node::SharedNode<S>, interval: u64, verbose: bool) {
    if interval == 0 {
        return;
    }
    loop {
        std::thread::sleep(std::time::Duration::from_secs(interval));
        let node = node.write().unwrap();
        node.get_chain().grow();
        if verbose {
            println!("New block")
        }
    }
}
