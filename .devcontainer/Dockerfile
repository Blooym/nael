FROM rust:1-bookworm

# Add non-root user
RUN adduser --disabled-password --gecos "" dev

# Install Rust components
RUN rustup component add rustfmt clippy

# Setup shell environment
ENV DALAMUD_HOME=/home/dev/.config/nael/current
RUN echo "alias nael='cargo run'" >> /home/dev/.bash_aliases