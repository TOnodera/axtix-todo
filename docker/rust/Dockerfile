FROM rust:latest

RUN useradd -m -u 1000 -s /bin/bash rust 
RUN rustup component add rustfmt
RUN apt update -y
RUN cargo install cargo-watch
RUN chown -R rust:rust /usr/local/cargo

USER rust