use jsonrpc_core::IoHandler;
use jsonrpc_http_server::ServerBuilder;

mod abci;
mod cli;
mod data;
mod init;
mod node;
mod server;
mod store;
mod test_node;
mod chain;

use server::Rpc;

fn main() {
    let args = cli::get_args();
    let id = String::from("flintheart");
    let mut node = node::Node::new(id.clone());
    init::init(&mut node, &id);
    let server = server::Server::new(args.verbose, node);
    let mut io = IoHandler::new();
    io.extend_with(server.to_delegate());

    println!("Starting JsonRPC");
    let server = ServerBuilder::new(io)
        .start_http(&format!("127.0.0.1:{}", args.port).parse().unwrap())
        .expect("Unable to start RPC server");

    server.wait();
}
