# Notes

## Light clients 

In the IBC protocol, a light client of chain A is a module that has the capability to verify the authenticity of incoming blocks from chain B, it does this by storing information about B state in its own storage.

There are light clients at multiple level:
- Chain A has a light client that verify incoming blocks from chain B
- Chain B has a light client that verify incoming blocks from chain A
- The ICB relayer has both a light client for A and a light client for B

Light clients may use different type of storage, for instance chain A's light client use the chain storage itself, while the relayer's light client probably uses some kind of persistent local storage.

A light node is a process running a light clients, it can be used as a standalone daemon.

A light client communicate with the blockhain (i.e. talks to a full node) through the RPC interface.

Useful resources:
- [Light client verification](https://github.com/informalsystems/tendermint-rs/blob/master/docs/spec/lightclient/verification/verification.md)
- [Foreign interface](https://github.com/informalsystems/tendermint-rs/blob/master/docs/spec/lightclient/verification/verification.md#used-remote-functions).


## Light nodes

A light node is a process running a light client, it exposes a jsonrpc interface.

I couldn't manage to actually run a light client (see error below + 'header does not match' using a single localhost (go) terndermint full node.

However, looking at the [code](https://github.com/informalsystems/tendermint-rs/blob/master/light-node/src/rpc.rs), it seems that it actually support only two queries:

- `state`
- `status`

## Mocked block-chain

The block-chain is queried by its light-client, it needs to implement the two following functions:

- commit
- validators

## Is this an error?

### Light node fails to parse full node response

The light node fails on initialization due to a parse error while parsing the full node RPC response.

**Steps to reproduce:**

Start a full node using ibc-rs example container:

```sh
docker run --rm -d -p 26656:26656 -p 26657:26657 informaldev/chain_a
```

Get a header hash:

```sh
curl -X GET "http://localhost:26657/block?height=2" -H  "accept: application/json" | jq .result.block_id.hash
```

And initialize the light node (version 0.16.0) :

```sh
cargo run --  initialize  2 <your_header_hash>
```

**Expected behavior**

The initialization succeed.

**Current behavior**

A parse error is raised:

```text
could not retrieve trusted header: Parse error. Invalid JSON: invalid type: string "1", expected u64 at line 17 column 24 (code: -32700)
```

By looking at the message exchange through wireshark, the faulty json is (with stripped fields):

```json
{
  "jsonrpc": "2.0",
  "result": {
    "signed_header": {
      "header": {
        "last_block_id": {
            "parts": {
                "total": "1",
	    }
        }
    }
}
```

