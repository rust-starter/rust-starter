FROM rust:1.43

# Copy project source code
WORKDIR /usr/src/rustapp
COPY . .

# Print versions information
RUN rustc --version
RUN cargo --version
RUN rustup --version

# Build the project
RUN cargo build
