# IBC-mock logs

A journal to help me remember what happens on IBC-mock.

### October 5

Successfully runned a light-node against the mocked Tendermint node:

- For now the blochain only has a single block.
- I had to use the default configuration of the light node, otherwise a missing witnesses error is sent. The config defines a trust threshold, that is probably why it fixed the issue.
- Work on the JsonRPC interface will be posed until the testgen lightblocks generation is availlable.

**Next task**: Have the `abci` interface up and running.

- It seems that it's possible to query the `abci` interface through the same JsonRPC interface that I used before, if that's the case I will reuse the same server.
- Relevent specification can be found in [ICS 24 (Host Requirements)](https://github.com/cosmos/ics/tree/master/spec/ics-024-host-requirements).
- From ICS 24 values stores **MUST** use the canonical encoding, proto3 files are provided with the spec.

### October 7

- Added support for Merk store.
- Handle `/abci_query` and `/abci_info`

To resolve:

- How does the relayer submit transaction? Didn't see any use of the rpc client for that.
- tendermint-rpc is build for client support, but does not expose request internal values (nedded to implement a server)
- How to handle the 'height' param in queries?

### October 12

- Fixed issue while communicating with relayer-cli.

### October 16

- Refactor store, use a simple nested hashmap for now.
- Refactor server to give it access to the shared store.
- Change dependencies to tendermint-rs master branch (after merge that exposes requests types).
- Started implementing `ClientReader` and `ClientKeeper` traits.

### October 18

- Working on `ClientReader` and `ClientKeeper` traits.

_Questions_:
- How to serialize values stored? For instance I can't find how to serialize `ClientType`, but an universal serialization is needed because of ABCI queries.
- How to create a raw `Any` type? Where are the urls?
- How to import tendermint properly?

### October 19

- Updated dependencies to tendermint v0.17.0-rc.
- Finished client related traits + add unit testing.

_Other_:
- I'm having trouble with package versionning, that is due to the fact that I need to use the git version of Tendermint (I depend on the test crate with is not on crate.io), AND ibc which use the crate.io version.


_What do we need from blocks?_:
- Header
- Validator set

Remarks:

- ConnectionKeeper function `store_connection` takes a `&ConnectionEnd`, but for ClientKeeper values stored are owned.

### October 21

- Introduce Node struct, that holds both a private and provable store + a chain.
- Migrate Keeper implementations from Store to Node.

### October 22

- Done with refactoring
- Used interior mutability to allow concurrent use of `Node` by the RPC server.
- Added the `boradcast_tx_commit` endpoint, but without implementation (blocked by ICS26Enveloppe which is not serializable).

### October 23

- Added initialization of node store.
- Implementing Connections Keeper and Reader

Questions:
- Is there a protobuf type for Connections? I'm serialyzing a Json list for now...

### October 26

- Implementing ConnectionReader for Node.
