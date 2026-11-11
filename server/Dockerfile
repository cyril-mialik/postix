FROM rust:1.89-slim AS builder

WORKDIR /app

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./
COPY server/Cargo.toml ./server/
COPY client/Cargo.toml ./client/

RUN mkdir server/src && echo "fn main() {}" > server/src/main.rs
RUN mkdir client/src && echo "fn main() {}" > client/src/main.rs
RUN cargo build --release -p server

COPY server/src ./server/src
COPY server/migrations ./server/migrations
COPY server/.sqlx ./server/.sqlx

ENV SQLX_OFFLINE=true

RUN rm -rf target/release/deps/server-* target/release/server
RUN cargo build --release -p server

FROM debian:bookworm-slim

WORKDIR /app

RUN apt-get update && apt-get install -y \
    libssl3 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

RUN useradd -m -u 1000 appuser && chown -R appuser:appuser /app
USER appuser

COPY --from=builder /app/target/release/server /app/server
COPY --from=builder /app/server/migrations /app/migrations

EXPOSE 8080
CMD ["./server"]
