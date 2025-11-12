.PHONY: build install clean test

build:
	cargo build --release

install:
	cargo install --path .

clean:
	cargo clean

test:
	cargo test
