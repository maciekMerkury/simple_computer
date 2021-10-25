default: test

.PHONY: test run full_build clean

all: full_build test

test:
	cargo test

run: 
	cargo run

full_build:
	cargo build
	cargo build --release

clean:
	cargo clean

