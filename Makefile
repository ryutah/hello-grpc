.PHONY: all

CURDIR := $(shell pwd)

help: ## Print this help
	@echo 'Usage: make [target]'
	@echo ''
	@echo 'Targets:'
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}' $(MAKEFILE_LIST)

build: ## Build server, clients
	cd ./server/rust && cargo build --release
	go generate github.com/ryutah/hello-grpc/client/go
	go build -o ./client/go/bin/client github.com/ryutah/hello-grpc/client/go

serve_rust: ## Start Rust server
	./server/rust/target/release/hello-grpc
