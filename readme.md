# IBC Mock

## TODO:

1. Stores Mock parameters (e.g. `client_def`, `client_type`...)
	- See icb modules
2. Setup an HTTP server
	- Let's use Hyper
3. Deserialize IBC protobuf queries
4. Serialize response

**Storage**:
The light client relies on two functions from Tendermint RPC:
- **commit**
- **validators**

See [Used Remote Functions](https://github.com/informalsystems/tendermint-rs/blob/master/docs/spec/lightclient/verification/verification.md#used-remote-functions)

To implement RPC the `[terdermint_rpc](https://github.com/informalsystems/tendermint-rs/tree/master/rpc)` crate can be helpful. This crate exposes requests and response structs deriving Serde's Deserialize and Serialize traits.

## Questions:

- What are the server parameters?
- How do I deserialize? `try_from_raw`?
- Where are messages definitions?
- What is the client used for?

## Useful links:
- [IBC modules](https://github.com/informalsystems/ibc-rs/tree/master/modules)
- [Light client](https://github.com/informalsystems/tendermint-rs/tree/master/light-client)
- [Merkle tree crate](https://docs.rs/merkletree/0.21.0/merkletree/)
- [HTTP crate](https://docs.rs/hyper/0.13.7/hyper/index.html)

