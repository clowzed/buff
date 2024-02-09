# Step 1: Build the Rust application
FROM rust:latest AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/buff .
RUN apt-get update && apt install -y openssl
RUN \
    apt-get update && \
    apt-get install -y ca-certificates && \
    apt-get clean
RUN mkdir images
CMD ["./buff"]
