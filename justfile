run-test TEST:
	cargo test --test {{TEST}}

debug TEST:
	cargo test --test {{TEST}} --features debug

run-tests:
	cargo test 

bench: 
	cargo bench 

lint:
	cargo clippy 

clean:
	cargo clean
	find . -type f -name "*.orig" -exec rm {} \;
	find . -type f -name "*.bk" -exec rm {} \;
	find . -type f -name ".*~" -exec rm {} \;

build-docker:
	docker run -v "$PWD":/build \
	-v $PWD/.cache/.cargo/git:/root/.cargo/git \
	-v $PWD/.cache/.cargo/registry:/root/.cargo/registry \
	--entrypoint cargo \
	fredrikfornwall/rust-static-builder \
	run  --target x86_64-unknown-linux-musl
