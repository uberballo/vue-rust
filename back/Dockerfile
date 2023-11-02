FROM rust:1.73 AS builder
COPY . .
RUN cargo build --release

FROM debian:stable-slim
COPY --from=builder ./target/release/back ./target/release/back
EXPOSE 3000
CMD ["/target/release/back"]