use tendermint::block::header::{Header, Version};
use tendermint::{
    abci, account, block, chain, consensus, evidence, public_key, time, validator, vote,
};

/// Returns the `Info` about a factice validator.
fn _get_validator() -> validator::Info {
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
    let validators_hash = validator::Set::new(vec![_get_validator()]).hash();
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
