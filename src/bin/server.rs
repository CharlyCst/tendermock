use jsonrpc_core;
use jsonrpc_core::{serde_json, Error as JsonError, IoHandler, Params, Result as JsonResult};
use jsonrpc_derive::rpc;
use jsonrpc_http_server::ServerBuilder;
use tendermint_rpc::endpoint::{
    commit::Request as CommitRequest, /*commit::Response as CommitResponse, */
    validators::Request as ValidatorsRequest,
};

use tendermint::account;
use tendermint::block;
use tendermint::chain;
use tendermint::block::header::{Header, Version};
use tendermint::consensus;
use tendermint::evidence;
use tendermint::public_key;
use tendermint::time;
use tendermint::validator;
use tendermint::vote;
use tendermint::abci;

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
        .start_http(&"127.0.0.1:26657".parse().unwrap())
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

/// Returns the `Info` about a factice validator.
fn get_validator() -> validator::Info {
    let key = public_key::Ed25519::from_bytes(&[
        215, 90, 152, 1, 130, 177, 10, 183, 213, 75, 254, 211, 201, 100, 7, 58, 14, 225, 114, 243,
        218, 166, 35, 37, 175, 2, 26, 104, 247, 7, 81, 26,
    ])
    .unwrap();
    let voting_power = vote::Power::new(10);

    validator::Info::new(public_key::PublicKey::Ed25519(key), voting_power)
}

/// Returns the `Params` of a factice concensus.
fn _get_concensus() -> consensus::Params {
    let block = block::Size {
        max_bytes: 2048,
        max_gas: 64,
    };
    let evidence = evidence::Params {
        max_age_num_blocks: 5,
        max_age_duration: evidence::Duration(std::time::Duration::new(5, 0)),
    };
    let validator = consensus::params::ValidatorParams {
        pub_key_types: vec![public_key::Algorithm::Ed25519],
    };

    consensus::Params {
        block,
        evidence,
        validator,
    }
}

/// TODO: build a block
fn _get_bloc() -> block::Block {
    let version = Version { block: 0, app: 0 };
    let chain_id = chain::Id::from("zephyr");
    let height = block::Height::from(1);
    let time = time::Time::now();
    let validators_hash = validator::Set::new(vec![get_validator()]).hash();
    let next_validators_hash = validators_hash.clone();
    let consensus_hash = validators_hash.clone();
    let app_hash = vec![b'a'; 10];
    let proposer_address = account::Id::new([b'a'; 20]);

    let header = Header {
        version,
        chain_id,
        height,
        time,
        last_block_id: None,
        last_commit_hash: None,
        data_hash: None,
        validators_hash,
        next_validators_hash,
        consensus_hash,
        app_hash,
        last_results_hash: None,
        evidence_hash: None,
        proposer_address,
    };

    block::Block {
        header,
        data: abci::transaction::Data::new(vec![]),
        evidence: evidence::Data::new(vec![]),
        last_commit: None,
    }
}
