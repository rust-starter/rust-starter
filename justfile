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
	docker build -t rust-starter -f docker/Dockerfile .
	mv .dockerignore docker/.dockerignore

docker-version:
	docker run -t rust-starter rustc --version
	docker run -t rust-starter cargo --version
	docker run -t rust-starter rustup --version

docker-run:
	docker run -t rust-starter cargo build --all --all-targets 

docker-test:	
	docker run -t rust-starter cargo test --all
