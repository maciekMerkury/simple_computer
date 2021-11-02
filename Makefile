default: test

.PHONY: test run full_build clean clear fmt

all: test full_build fmt

clear:
	clear

test:
	cargo test

run: 
	cargo run

full_build:
	cargo build
	cargo build --release

clean:
	cargo clean

fmt:
	cargo fmt

