use futures::future::FutureExt;
use futures::try_join;
use tokio;
use warp;
use warp::Filter as _;

mod abci;
mod avl;
mod chain;
mod cli;
mod config;
mod data;
mod grpc;
mod init;
mod jrpc;
mod node;
mod store;
mod test_node;

use jrpc::{Jrpc, Ws};

pub const WEBSOCKET_PATH: &str = "websocket";

fn main() {
    let args = cli::get_args();
    let config = if let Some(config_path) = args.config {
        config::Config::load(config_path)
    } else {
        config::Config::default()
    };
    let node = node::Node::new(&config);
    let mut node = node.shared();
    init::init(&mut node, &config);

    // Automatically grow the chain
    let block_interval = args.block;
    let verbose = args.verbose;
    let growth_node = node.clone();
    std::thread::spawn(move || schedule_growth(growth_node, block_interval, verbose));

    // JsonRPC server
    let jrpc_api = warp::path::end().and(Jrpc::new(args.verbose, node.clone()));
    let ws = warp::path(WEBSOCKET_PATH).and(Ws::new());
    let jrpc_server = warp::serve(jrpc_api.or(ws))
        .run(
            format!("127.0.0.1:{}", args.json_port)
                .parse::<std::net::SocketAddr>()
                .expect("Invalid IP address or port"),
        )
        .then(|()| async { Ok(()) });

    // gRPC server
    let addr = format!("[::1]:{}", &args.grpc_port).parse().unwrap();
    let grpc_server = grpc::new(node).serve(addr);

    // Start servers
    println!("[gRPC] Starting server on port: {}", &args.grpc_port);
    println!("[JsonRPC] Starting server on port: {}", &args.json_port);
    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(async { try_join!(jrpc_server, grpc_server) })
        .unwrap();
}

/// Push a new block on the chain every `interval` seconds.
fn schedule_growth<S: store::Storage>(node: node::SharedNode<S>, interval: u64, verbose: bool) {
    if interval == 0 {
        return;
    }
    // Add a block as if it was added last midnight (UTC).
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let midnight = now - (now % 86_400);
    let node_ref = node.write();
    node_ref.get_chain().grow_at(midnight);
    drop(node_ref);
    if verbose {
        display_last_block(&node);
    }
    loop {
        std::thread::sleep(std::time::Duration::from_secs(interval));
        let node_ref = node.write();
        node_ref.get_chain().grow();
        drop(node_ref);
        if verbose {
            display_last_block(&node);
        }
    }
}

/// Displays the last block of the node's chain.
fn display_last_block<S: store::Storage>(node: &node::SharedNode<S>) {
    let node = node.read();
    let block = node.get_chain().get_block(0).unwrap();
    let header = block.signed_header.header;
    println!(
        "[Chain] Height: {} - Hash: {}",
        header.height,
        &header.hash()
    );
}
