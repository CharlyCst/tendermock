use tendermint::{abci, block, evidence, validator};

use tendermint_testgen;
use tendermint_testgen::Generator;

/// Builds a header
fn _get_header() -> tendermint_testgen::Header {
    let validator = tendermint_testgen::Validator::new("validator_1");
    let mut header = tendermint_testgen::Header::new(&[validator]);
    header.time =
        Some(16000000); // TODO: find a valid timestamp
    header
}

/// Builds a header
pub fn _get_commit() -> tendermint_testgen::Commit {
    let header = _get_header();
    tendermint_testgen::Commit::new(header, 0)
}

/// Builds a block
pub fn _get_block() -> block::Block {
    block::Block {
        header: _get_header().generate().unwrap(),
        data: abci::transaction::Data::new(vec![]),
        evidence: evidence::Data::new(vec![]),
        last_commit: None,
    }
}

pub fn _get_signed_header() -> block::signed_header::SignedHeader {
    let header = _get_header().generate().unwrap();
    let commit = _get_commit().generate().unwrap();

    block::signed_header::SignedHeader { header, commit }
}

pub fn _get_validators() -> Vec<validator::Info> {
    let validator = tendermint_testgen::Validator::new("validator_1")
        .generate()
        .unwrap();
    vec![validator]
}
