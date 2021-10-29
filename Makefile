default: test

.PHONY: test run full_build clean clear

all: full_build test

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

