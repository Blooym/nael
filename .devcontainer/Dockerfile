FROM rust:1-bookworm

# Add non-root user
RUN adduser --disabled-password --gecos "" dev

# Install Rust components
RUN rustup component add rustfmt clippy