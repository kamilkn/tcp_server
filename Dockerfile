FROM rust:latest as builder
WORKDIR /usr/src/wisdom_server
COPY . .
RUN cargo build --release

FROM debian:buster-slim
COPY --from=builder /usr/src/wisdom_server/target/release/wisdom_server /usr/local/bin/wisdom_server
CMD ["wisdom_server"]
