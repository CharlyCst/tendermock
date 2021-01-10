//! # Tendermock
//!
//! A tendermint mocked node.
use futures::try_join;
use tokio;

mod abci;
mod avl;
mod chain;
mod cli;
mod config;
mod grpc;
mod init;
mod jrpc;
mod node;
mod store;
mod test_node;

fn main() {
    // Parse cli arguments & initialize store
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
    let addr = format!("127.0.0.1:{}", args.json_port).parse().unwrap();
    let jrpc_server = jrpc::serve(node.clone(), args.verbose, addr);

    // gRPC server
    let addr = format!("[::1]:{}", &args.grpc_port).parse().unwrap();
    let grpc_server = grpc::serve(node, args.verbose, addr);

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
    node.grow();
    if verbose {
        display_last_block(&node);
    }
    if interval == 0 {
        return;
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
