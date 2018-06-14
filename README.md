# hello-grpc
gRPC examples

# Build

```console
$ make build
```

# Start server
Execute server that listen addr on `127.0.0.1:8080`

## Rust gRPC Server
```console
$ make serve_rust
```

# Execute Client
Execute client to send request server listen on `127.0.0.1:8080`

## Go Client
```console
$ <PROJECT_ROOT>/client/go/bin/client <REQUEST_MESSAGE>
```
