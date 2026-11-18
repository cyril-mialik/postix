FROM rust:1.100-slim AS builder

WORKDIR /app

RUN rustup target add wasm32-unknown-unknown \
    && cargo install trunk --version 0.22.1 --locked

COPY . .

WORKDIR /app/client
RUN trunk build --release

FROM scratch AS exportor
COPY --from=builder /app/client/dist /dist
