
.PHONY: test

./target/brunnr: ./src/*.rs
	cargo build
