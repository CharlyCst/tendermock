use tendermint::{abci, block, evidence, validator};

use tendermint_testgen;
use tendermint_testgen::Generator;

/// Builds a header
fn _get_header() -> tendermint_testgen::Header {
    let validator = tendermint_testgen::Validator::new("validator_1");
    tendermint_testgen::Header::new(&[validator])
}

/// Builds a block
fn _get_bloc() -> block::Block {
    block::Block {
        header: _get_header().generate().unwrap(),
        data: abci::transaction::Data::new(vec![]),
        evidence: evidence::Data::new(vec![]),
        last_commit: None,
    }
}

/// Builds a commit
fn _get_commit() -> tendermint_testgen::Commit {
    let header = _get_header();
    tendermint_testgen::Commit::new(header, 0)
}

pub fn _get_signed_header() -> block::signed_header::SignedHeader {
    let header = _get_header().generate().unwrap();
    let commit = _get_commit().generate().unwrap();

    block::signed_header::SignedHeader { header, commit }
}

pub fn _get_validators() -> Vec<validator::Info> {
    let validator = tendermint_testgen::Validator::new("validator_1").generate().unwrap();
    vec![validator]
}
