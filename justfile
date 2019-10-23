#!/bin/bash

package_name := `sed -En 's/name[[:space:]]*=[[:space:]]*"([^"]+)"/\1/p' Cargo.toml | head -1`
package_version := `sed -En 's/version[[:space:]]*=[[:space:]]*"([^"]+)"/\1/p' Cargo.toml | head -1`

run-test TEST:
	cargo test --test {{TEST}}

debug TEST:
	cargo test --test {{TEST}} --features debug

run-tests:
	cargo test --all

bench:
	cargo bench

lint:
	cargo clippy

clean:
	cargo clean
	find . -type f -name "*.orig" -exec rm {} \;
	find . -type f -name "*.bk" -exec rm {} \;
	find . -type f -name ".*~" -exec rm {} \;	

docker-build:
	mv docker/.dockerignore .dockerignore
	docker build -t {{package_name}}_{{package_version}} -f docker/Dockerfile .
	mv .dockerignore docker/.dockerignore

docker-version:
	docker run -t {{package_name}}_{{package_version}} rustc --version
	docker run -t {{package_name}}_{{package_version}} cargo --version
	docker run -t {{package_name}}_{{package_version}} rustup --version

docker-run:
	docker run -t {{package_name}}_{{package_version}} cargo run 

docker-test:	
	docker run -t {{package_name}}_{{package_version}} cargo test --all
