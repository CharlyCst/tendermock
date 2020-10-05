**October 5**

Successfully runned a light-node against the mocked Tendermint node:

- For now the blochain only has a single block.
- I had to use the default configuration of the light node, otherwise a missing witnesses error is sent. The config defines a trust threshold, that is probably why it fixed the issue.
- Work on the JsonRPC interface will be posed until the testgen lightblocks generation is availlable.

