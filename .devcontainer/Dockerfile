FROM rust:1.76-bookworm
RUN adduser --disabled-password --gecos "" dev

# Setup shell environment
RUN echo "alias nael='RUSTFLAGS=-Awarnings cargo run'" >> /home/dev/.bash_aliases
RUN apt-get update && apt-get -y install gcc-aarch64-linux-gnu

# Install Rust components
RUN rustup component add rustfmt clippy

# De-elevate and install tooling.
USER dev
RUN cargo install cargo-dist