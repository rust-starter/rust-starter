FROM ekidd/rust-musl-builder:latest as Builder


# Copy project source code
COPY . .
ADD --chown=rust:rust . .

# Print versions information
RUN rustc --version
RUN cargo --version
RUN rustup --version

# Build the project
RUN cargo build --release

FROM alpine:latest
RUN apk --no-cache add ca-certificates
COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/rust-starter /usr/local/bin/rust-starter
CMD ["/usr/local/bin/rust-starter"]
