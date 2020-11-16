// use ibc::ics26_routing::handler::deliver_tx;
use jsonrpc_core::{serde_json, Error as JsonError, Params, Result as JsonResult};
use jsonrpc_derive::rpc;
use std::sync::RwLock;
use tendermint::block::Height;
use tendermint_rpc::endpoint::{
    abci_info::Request as AbciInfoRequest, abci_info::Response as AbciInfoResponse,
    abci_query::Request as AbciQueryRequest, abci_query::Response as AbciQueryResponse,
    block::Request as BlockRequest, block::Response as BlockResponse,
    broadcast::tx_commit::Request as BroadcastTxCommitRequest,
    broadcast::tx_commit::Response as BroadcastTxCommitResponse, commit::Request as CommitRequest,
    commit::Response as CommitResponse, validators::Request as ValidatorsRequest,
    status::Request as StatusRequest, status::Response as StatusResponse,
    validators::Response as ValidatorResponse,
};

use crate::abci;
use crate::node;
use crate::store;
use crate::chain::to_full_block;

const PUBLICK_KEY: &str =
        "4A25C6640A1F72B9C975338294EF51B6D1C33158BB6ECBA69FBC3FB5A33C9DCE";

#[rpc(server)]
pub trait Rpc {
    #[rpc(name = "block", params = "raw")]
    fn block(&self, req: Params) -> JsonResult<BlockResponse>;

    #[rpc(name = "commit", params = "raw")]
    fn commit(&self, req: Params) -> JsonResult<CommitResponse>;

    #[rpc(name = "validators", params = "raw")]
    fn validators(&self, req: Params) -> JsonResult<ValidatorResponse>;

    #[rpc(name = "abci_info", params = "raw")]
    fn abci_info(&self, req: Params) -> JsonResult<AbciInfoResponse>;

    #[rpc(name = "abci_query", params = "raw")]
    fn abci_query(&self, req: Params) -> JsonResult<AbciQueryResponse>;

    #[rpc(name = "status", params ="raw")]
    fn status(&self, req: Params) -> JsonResult<StatusResponse>;

    #[rpc(name = "broadcast_tx_commit", params = "raw")]
    fn broadcast_tx_commit(&self, req: Params) -> JsonResult<BroadcastTxCommitResponse>;
}

/// A JsonRPC server.
pub struct Server<S: store::Storage> {
    verbose: bool,
    node: RwLock<node::Node<S>>,
}

impl<S: store::Storage> Server<S> {
    pub fn new(verbose: bool, node: node::Node<S>) -> Self {
        let node = RwLock::new(node);
        Server { verbose, node }
    }
}

impl<S: 'static + store::Storage + Sync + Send> Rpc for Server<S> {
    /// JsonRPC /block endpoint.
    fn block(&self, req: Params) -> JsonResult<BlockResponse> {
        let req: BlockRequest = parse(req)?;
        if self.verbose {
            println!("JsonRPC /block      {:?}", req);
        }
        let height = match req.height {
            None => 0,
            Some(height) => height.into(),
        };
        let node = self.node.read().unwrap();
        let block = node
            .get_chain()
            .get_block(height)
            .ok_or_else(|| JsonError::invalid_request())?;
        let tm_block = to_full_block(block);
        let hash = tm_block.header.hash();
        let block_response  = BlockResponse {
            block_id: tendermint::block::Id {
                part_set_header: tendermint::block::parts::Header::new(1, hash.clone()),
                hash,
            },
            block: tm_block,
        };
        Ok(block_response)
    }

    /// JsonRPC /commit endpoint.
    fn commit(&self, req: Params) -> JsonResult<CommitResponse> {
        let req: CommitRequest = parse(req)?;
        if self.verbose {
            println!("JsonRPC /commit     {:?}", req);
        }
        let height = match req.height {
            None => 0,
            Some(height) => height.into(),
        };
        let node = self.node.read().unwrap();
        let block = node
            .get_chain()
            .get_block(height)
            .ok_or_else(|| JsonError::invalid_request())?;
        let signed_header = block.signed_header;
        let commit_response = CommitResponse {
            signed_header,
            canonical: false,
        };
        Ok(commit_response)
    }

    /// JsonRPC /validators endpoint.
    fn validators(&self, req: Params) -> JsonResult<ValidatorResponse> {
        let req: ValidatorsRequest = parse(req)?;
        if self.verbose {
            println!("JsonRPC /validators {:?}", req);
        }
        let node = self.node.read().unwrap();
        let block = node
            .get_chain()
            .get_block(req.height.into())
            .ok_or_else(|| JsonError::invalid_request())?;
        let validators = block.validators.validators().clone();
        let validators_responde = ValidatorResponse {
            block_height: Height::from(1 as u32),
            validators,
        };
        Ok(validators_responde)
    }

    /// JsonRPC /status endpoint.
    fn status(&self, req: Params) -> JsonResult<StatusResponse> {
        let _req: StatusRequest = parse(req)?;
        let node = self.node.read().unwrap();
        let node_info = node.get_info().clone();
        let sync_info = node.get_sync_info();
        let validator_info = tendermint::validator::Info {
            address: tendermint::account::Id::new([41; 20]),
            pub_key: tendermint::public_key::PublicKey::from_raw_ed25519(&hex::decode(PUBLICK_KEY).unwrap()).unwrap(),
            voting_power: tendermint::vote::Power::new(1),
            proposer_priority: None,
        };
        let status_response = StatusResponse {
            node_info,
            sync_info,
            validator_info,
        };
        Ok(status_response)
    }

    /// JsonRPC /abci_info endpoint.
    fn abci_info(&self, req: Params) -> JsonResult<AbciInfoResponse> {
        let req: AbciInfoRequest = parse(req)?;
        if self.verbose {
            println!("JsonRPC /abci_info  {:?}", req);
        }
        let abci_info_response = AbciInfoResponse {
            response: abci::get_info(),
        };
        Ok(abci_info_response)
    }

    /// JsonRPC /abci_query endpoint.
    fn abci_query(&self, req: Params) -> JsonResult<AbciQueryResponse> {
        let req: AbciQueryRequest = parse(req)?;
        if self.verbose {
            println!("JsonRPC /abci_query {:?}", req);
        }
        let node = self.node.read().unwrap();
        let abci_query_response = AbciQueryResponse {
            response: abci::handle_query(req, &node),
        };
        Ok(abci_query_response)
    }

    /// JsonRPC /broadcast_tx_commit endpoint.
    fn broadcast_tx_commit(&self, req: Params) -> JsonResult<BroadcastTxCommitResponse> {
        let _req: BroadcastTxCommitRequest = parse(req)?;
        let node = self.node.write().unwrap();
        node.get_chain().grow();
        todo!();
    }
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
