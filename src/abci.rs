use tendermint::abci::{Code, Log};
use tendermint::block;
use tendermint_rpc::endpoint::{
    abci_info::AbciInfo, abci_query::AbciQuery, abci_query::Request as AbciQueryRequest,
};

use crate::store::Storage;

pub fn get_info() -> AbciInfo {
    AbciInfo {
        data: "data_placeholder".to_string(),
        version: "v0.16.0".to_string(),
        app_version: 1,
        last_block_height: tendermint::block::Height::from(1 as u32),
        last_block_app_hash: vec![],
    }
}

pub fn handle_query<T: Storage>(query: AbciQueryRequest, store: &T) -> AbciQuery {
    // let path = query.
    AbciQuery {
        code: Code::Err(1),
        log: Log::from("Does not exist"),
        info: "abci_query is not yet implemented".to_string(),
        index: 0,
        key: vec![],
        value: vec![],
        proof: None,
        height: block::Height::from(1 as u32),
        codespace: "codespace".to_string(),
    }
}
