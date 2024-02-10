FROM rust:1.76-bookworm
RUN adduser --disabled-password --gecos "" dev

# Install Rust components
RUN rustup component add rustfmt clippy

# De-elevate and install tooling.
USER dev
RUN cargo install cargo-dist