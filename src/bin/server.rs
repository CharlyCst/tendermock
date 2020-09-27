use jsonrpc_core;
use jsonrpc_core::{serde_json, Error as JsonError, IoHandler, Params, Result as JsonResult};
use jsonrpc_derive::rpc;
use jsonrpc_http_server::ServerBuilder;
use tendermint_rpc::endpoint::{
    commit::Request as CommitRequest, /*commit::Response as CommitResponse, */
    validators::Request as ValidatorsRequest,
};

#[rpc(server)]
pub trait Rpc {
    #[rpc(name = "commit", params = "raw")]
    fn commit(&self, req: Params) -> JsonResult<String>;

    #[rpc(name = "validators", params = "raw")]
    fn validators(&self, req: Params) -> JsonResult<String>;
}

struct Server;

impl Rpc for Server {
    fn commit(&self, req: Params) -> JsonResult<String> {
        let req: CommitRequest = parse(req)?;
        println!("{:?}", req);
        Ok("hello".to_string())
    }

    fn validators(&self, req: Params) -> JsonResult<String> {
        let req: ValidatorsRequest = parse(req)?;
        println!("{:?}", req);
        Ok("hello".to_string())
    }
}

fn main() {
    let server = Server {};
    let mut io = IoHandler::new();
    io.extend_with(server.to_delegate());

    let server = ServerBuilder::new(io)
        .start_http(&"127.0.0.1:3030".parse().unwrap())
        .expect("Unable to start RPC server");

    server.wait();
}

/// Deserializes raw parameters.
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
