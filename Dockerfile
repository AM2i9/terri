FROM rust:latest

WORKDIR /terri

COPY . .

RUN cargo build --release

ENTRYPOINT ["/terri/target/release/terri"]