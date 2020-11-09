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

graph:
    rm -f cargo-graph.dot
    rm -f cargo-graph.png
    cargo graph --optional-line-style dashed --optional-line-color red --optional-shape box --build-shape diamond --build-color green --build-line-color orange > cargo-graph.dot
    dot -Tpng > cargo-graph.png cargo-graph.dot

clean:
	cargo clean
	find . -type f -name "*.orig" -exec rm {} \;
	find . -type f -name "*.bk" -exec rm {} \;
	find . -type f -name ".*~" -exec rm {} \;	

docker:
    mv docker/.dockerignore .dockerignore
    docker build -t {{package_name}}_{{package_version}} -f docker/Dockerfile .
    mv .dockerignore docker/.dockerignore
