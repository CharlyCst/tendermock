# IBC Mock

## TODO:

- Setup an HTTP server
	- Let's use Hyper
- Deserialize IBC jsonRPC queries
- Serialize response

**Storage**:
The light client relies on two functions from Tendermint RPC:
- **commit**
- **validators**

See [Used Remote Functions](https://github.com/informalsystems/tendermint-rs/blob/master/docs/spec/lightclient/verification/verification.md#used-remote-functions)

To implement RPC the `[terdermint_rpc](https://github.com/informalsystems/tendermint-rs/tree/master/rpc)` crate can be helpful. This crate exposes requests and response structs deriving Serde's Deserialize and Serialize traits.

## Questions:

- JSON RPC request wrapper is not public (see [code](https://docs.rs/tendermint-rpc/0.16.0/src/tendermint_rpc/request.rs.html#1-50))
- tendermint-rpc expects a string for the `height` parameter, but the light client doc says that is send a number (see [doc](https://github.com/informalsystems/tendermint-rs/blob/master/docs/spec/lightclient/verification/verification.md#used-remote-functions))


## Useful links:
- [IBC modules](https://github.com/informalsystems/ibc-rs/tree/master/modules)
- [Light client](https://github.com/informalsystems/tendermint-rs/tree/master/light-client)
- [Merkle tree crate](https://docs.rs/merkletree/0.21.0/merkletree/)
- [HTTP crate](https://docs.rs/hyper/0.13.7/hyper/index.html)

