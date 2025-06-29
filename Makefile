.PHONY: help build test test-no-run


help:
	@echo "Available commands:"
	@echo "  make build   		- Build the project"
	@echo "  make clean   		- Clean build artifacts"
	@echo "  make test-no-run   - Compile tests"
	@echo "  make test    		- Run tests"
	@echo "  make help    		- Show this help message"


test-no-run:
	cargo test --no-run --all-targets

test:
	cargo test --all-targets

build:
	cargo build