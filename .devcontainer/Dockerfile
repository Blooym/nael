FROM rust:1-bookworm

# Add non-root user
RUN adduser --disabled-password --gecos "" dev

# Install Rust components
RUN rustup component add rustfmt clippy

# Setup shell environment
ENV DALAMUD_HOME=/home/dev/.config/nael/active
RUN echo "alias nael='RUSTFLAGS=-Awarnings cargo run'" >> /home/dev/.bash_aliases