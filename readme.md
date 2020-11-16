# Tendermock

A fake Tendermind for testing purposes.

## How to use:

The fake node can be run with:

```sh
cargo run
```

A few options are available:

```
USAGE:
    tendermock [FLAGS] [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -v, --verbose    Verbode mode
    -V, --version    Prints version information

OPTIONS:
    -c, --config <config>    Path to json configuration file
    -p, --port <port>        JsonRPC port [default: 26657]
```

An example of a valid config can be found in `test/config.example.json`, which can be used like that:

```sh
cargo run -- -c test/config.example.json -v
```

## Sending queries

A few example queries are available in `./queries`, the node can easily be queried using curl:

```
curl -X POST -H 'Content-Type: application/json' -d @queries/block.json 127.0.0.1:26657/ | jq
```

