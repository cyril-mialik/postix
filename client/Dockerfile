FROM rust:1.89-slim AS builder

WORKDIR /app

RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk

COPY . .

RUN trunk build --release

FROM scratch AS exportor
COPY --from=builder /app/dist /dist
