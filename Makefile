.PHONY: all

CURDIR := $(shell pwd)

help: ## Print this help
	@echo 'Usage: make [target]'
	@echo ''
	@echo 'Targets:'
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}' $(MAKEFILE_LIST)

build: ## Build server, clients
	cd ./server/rust && cargo build --release
	cd ./server/rs && cargo build --release
	go generate github.com/ryutah/hello-grpc/client/go
	go generate github.com/ryutah/hello-grpc/server/go
	go build -o ./client/go/bin/client github.com/ryutah/hello-grpc/client/go
	go build -o ./server/go/bin/server github.com/ryutah/hello-grpc/server/go

serve_rust: ## Start grpc-rust base Rust server
	./server/rust/target/release/hello-grpc

serve_rs: ## Start grpc-rs base Rust server
	./server/rs/target/release/rs

serve_go: ## Start grpc base Golang
	./server/go/bin/server

run_go_cli: ## Run golang client. Args: name = greet request name parameter.
	./client/go/bin/client ${name}
