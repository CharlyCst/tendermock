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


