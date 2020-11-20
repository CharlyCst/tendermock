use jsonrpc_core::IoHandler;
use jsonrpc_http_server::ServerBuilder;

mod abci;
mod avl;
mod chain;
mod cli;
mod config;
mod data;
mod init;
mod node;
mod server;
mod store;
mod test_node;

use server::Rpc;

fn main() {
    let args = cli::get_args();
    let config = if let Some(config_path) = args.config {
        config::Config::load(config_path)
    } else {
        config::Config::default()
    };
    let mut node = node::Node::new(&config);
    init::init(&mut node, &config);
    let server = server::Server::new(args.verbose, node);

    // Automatically grow the chain
    let node = server.get_node();
    std::thread::spawn(move || schedule_growth(node, 3));

    // Start the server
    println!("Starting JsonRPC");
    let mut io = IoHandler::new();
    io.extend_with(server.to_delegate());
    let server = ServerBuilder::new(io)
        .start_http(
            &format!("127.0.0.1:{}", args.port)
                .parse()
                .expect("Invalid IP address or port"),
        )
        .expect("Unable to start RPC server");

    server.wait();
}

/// Push a new block on the chain every `interval` seconds.
pub fn schedule_growth<S: store::Storage>(node: server::SharedNode<S>, interval: u64) {
    loop {
        std::thread::sleep(std::time::Duration::from_secs(interval));
        let node = node.write().unwrap();
        node.get_chain().grow();
    }
}
