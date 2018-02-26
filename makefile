SHELL := /bin/bash

all:
	cargo +nightly build --release --target wasm32-unknown-unknown
	mkdir -p site
	find target/wasm32-unknown-unknown/release/deps -type f -name "*.wasm" | xargs -I {} cp {} site/site.wasm
	python -m SimpleHTTPServer