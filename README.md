# DogeBot

> wow very doge

Upon successful token retrieval, will post a message to `#general`.

## Build

```bash
make build
```

## Run locally
Requires Dapr sidecar: https://docs.dapr.io/developing-applications/ides/intellij/
```bash
cargo run --package dogebot --bin dogebot -- -d
```

## Run locally with dapr

```bash
dapr run --app-id=dogebot --dapr-grpc-port 3500 cargo run -- -d
```