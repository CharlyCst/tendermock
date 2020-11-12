use ics23::{HashOp, InnerSpec, ProofSpec};

pub fn get_proof_spec() -> ProofSpec {
    ProofSpec {
        leaf_spec: None,
        inner_spec: Some(InnerSpec {
            child_order: vec![0, 2, 1],
            child_size: 0, // TODO what size?
            min_prefix_length: 0,
            max_prefix_length: 0,
            empty_child: vec![0, 20],
            hash: HashOp::Sha256.into(),
        }),
        max_depth: 0,
        min_depth: 0,
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn proof() {
    }
}
