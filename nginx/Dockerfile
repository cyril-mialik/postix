FROM rust:1.100-slim AS client_builder

WORKDIR /app

RUN rustup target add wasm32-unknown-unknown \
    && cargo install trunk --version 0.22.1 --locked

COPY . .

WORKDIR /app/client
RUN trunk build --release

FROM nginx:stable-alpine

COPY --from=client_builder /app/client/dist/ /usr/share/nginx/html/
COPY nginx/nginx.conf /etc/nginx/nginx.conf
COPY nginx/conf.d/ /etc/nginx/conf.d/

EXPOSE 80
