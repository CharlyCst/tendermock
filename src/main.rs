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

use jrpc::{Ws, Jrpc};

pub const WEBSOCKET_PATH: &str = "websocket";

fn main() {
    let args = cli::get_args();
    let config = if let Some(config_path) = args.config {
        config::Config::load(config_path)
    } else {
        config::Config::default()
    };
    let mut node = node::Node::new(&config);
    init::init(&mut node, &config);
    let node = node.shared();

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
    println!("Starting gRPC");
    let addr = format!("[::1]:{}", &args.grpc_port).parse().unwrap();
    let grpc_server = grpc::new(node).serve(addr);

    // Start servers
    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(async { try_join!(jrpc_server, grpc_server) })
        .unwrap();
}

/// Push a new block on the chain every `interval` seconds.
pub fn schedule_growth<S: store::Storage>(node: node::SharedNode<S>, interval: u64, verbose: bool) {
    if interval == 0 {
        return;
    }
    loop {
        std::thread::sleep(std::time::Duration::from_secs(interval));
        let node = node.write();
        node.get_chain().grow();
        if verbose {
            let block = node.get_chain().get_block(0).unwrap();
            let header = block.signed_header.header;
            println!("height: {} - hash: {}", header.height, &header.hash());
        }
    }
}
