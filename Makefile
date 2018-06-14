.PHONY: all

CURDIR := $(shell pwd)

help: ## Print this help
	@echo 'Usage: make [target]'
	@echo ''
	@echo 'Targets:'
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}' $(MAKEFILE_LIST)

build: ## Build server, clients
	cargo build --release
	protoc -I ./proto/ ./proto/helloworld.proto --go_out=plugins=grpc:client/helloworld
	go build -o ./client/bin/client github.com/ryutah/hello-grpc/client
