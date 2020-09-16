# Notes

## Light clients 

In the IBC protocol, a light client of chain A is a module that has the capability to verify the authenticity of incoming blocks from chain B, it does this by storing information about B state in chain A storage.

The light client is running as a separate process, it has its own data store and may query its chain's node, see [foreign interface](https://github.com/informalsystems/tendermint-rs/blob/master/docs/spec/lightclient/verification/verification.md#used-remote-functions).


