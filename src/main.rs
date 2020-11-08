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
        config::load(config_path)
    } else {
        config::default()
    };
    let mut node = node::Node::new(config.chain_id.clone());
    init::init(&mut node, &config);
    let server = server::Server::new(args.verbose, node);
    let mut io = IoHandler::new();
    io.extend_with(server.to_delegate());

    println!("Starting JsonRPC");
    let server = ServerBuilder::new(io)
        .start_http(&format!("127.0.0.1:{}", args.port).parse().expect("Invalid IP address or port"))
        .expect("Unable to start RPC server");

    server.wait();
}
