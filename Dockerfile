FROM rust
RUN cargo install cargo-edit
COPY src /opt/distrust/src
COPY Cargo.toml /opt/distrust/Cargo.toml
WORKDIR /opt/distrust
RUN cargo build
CMD ["cargo", "run"]