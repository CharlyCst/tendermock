use jsonrpc_core::IoHandler;
use jsonrpc_http_server::ServerBuilder;

mod abci;
mod blocks;
mod cli;
mod data;
mod module;
mod node;
mod server;
mod store;
mod test_node;

use server::Rpc;

fn main() {
    let args = cli::get_args();
    let store = store::InMemoryStore::new();
    let server = server::Server::new(args.verbose, store);
    let mut io = IoHandler::new();
    io.extend_with(server.to_delegate());

    println!("Starting JsonRPC");
    let server = ServerBuilder::new(io)
        .start_http(&format!("127.0.0.1:{}", args.port).parse().unwrap())
        .expect("Unable to start RPC server");

    server.wait();
}
