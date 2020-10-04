use jsonrpc_core;
use jsonrpc_core::{serde_json, Error as JsonError, IoHandler, Params, Result as JsonResult};
use jsonrpc_derive::rpc;
use jsonrpc_http_server::ServerBuilder;
use tendermint_rpc::endpoint::{
    commit::Request as CommitRequest, validators::Request as ValidatorsRequest,
};

mod blocks; // TODO
mod cli;

#[rpc(server)]
pub trait Rpc {
    #[rpc(name = "commit", params = "raw")]
    fn commit(&self, req: Params) -> JsonResult<String>;

    #[rpc(name = "validators", params = "raw")]
    fn validators(&self, req: Params) -> JsonResult<String>;
}

/// A JsonRPC server.
struct Server {
    verbose: bool,
}

impl Rpc for Server {
    /// JsonRPC /commit endpoint.
    fn commit(&self, req: Params) -> JsonResult<String> {
        let req: CommitRequest = parse(req)?;
        if self.verbose {
            println!("{:?}", req);
        }
        Ok("hello".to_string())
    }

    /// JsonRPC /validators endpoint.
    fn validators(&self, req: Params) -> JsonResult<String> {
        let req: ValidatorsRequest = parse(req)?;
        if self.verbose {
            println!("{:?}", req);
        }
        Ok("hello".to_string())
    }
}

fn main() {
    let args = cli::get_args();
    let server = Server {
        verbose: args.verbose,
    };
    let mut io = IoHandler::new();
    io.extend_with(server.to_delegate());

    let server = ServerBuilder::new(io)
        .start_http(&format!("127.0.0.1:{}", args.port).parse().unwrap())
        .expect("Unable to start RPC server");

    server.wait();
}

/// Deserializes raw parameters of a JsonRPC request.
fn parse<T>(params: Params) -> Result<T, JsonError>
where
    T: jsonrpc_core::serde::de::DeserializeOwned,
{
    let params = match params {
        Params::None => serde_json::Value::Null,
        Params::Array(vals) => serde_json::Value::Array(vals),
        Params::Map(val) => serde_json::Value::Object(val),
    };
    serde_json::from_value(params).map_err(|err| JsonError::invalid_params(err.to_string()))
}
