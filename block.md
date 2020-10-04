# Blocks

Just a small file to help me wrap my head around Tendermint blocks.

## The block structure

```rs
pub struct Block {
    pub header: Header,
    pub data: transaction::Data,
    pub evidence: evidence::Data,
    pub last_commit: Option<Commit>,
}
```

Let's build the first block: we'll put the `Commit` aside for now.

### Data

The data is a vector of `Transaction`s, those transactions are simply arbitrary bytes of data too (`Vec<u8>`).

### Evidence

A simple list of evidences, let's use an empty list.

### Header


The hard part...

```rs
pub struct Header {
    pub version:              Version,
    pub chain_id:             chain::Id,
    pub height:               block::Height,
    pub time:                 Time,
    pub last_block_id:        Option<block::Id>,
    pub last_commit_hash:     Option<Hash>,
    pub data_hash:            Option<Hash>,
    pub validators_hash:      Hash,
    pub next_validators_hash: Hash,
    pub consensus_hash:       Hash,
    pub app_hash:             Vec<u8>,
    pub last_results_hash:    Option<Hash>,
    pub evidence_hash:        Option<Hash>,
    pub proposer_address:     account::Id,
}
```

