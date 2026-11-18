# Postix

A social publishing app between Medium and Twitter, built with Rust, Yew, Actix Web, PostgreSQL, and Redis. All server endpoints are under `/api` (`/api/auth/...`, `/api/health`, and `/api/posts/...`).

### Local development
```bash
cp .env.example .env
docker compose up -d postgres redis

cargo run -p server
cd client && trunk serve
```

### Docker
```bash
# Start the API, frontend, PostgreSQL, and Redis
docker compose --profile prod up -d --build

# Optional local database UI
docker compose --profile dev-tools up -d pgadmin

# Stop containers; named database volumes are preserved
docker compose down
```

`trunk build --release` builds the frontend. The production Nginx image also builds and serves it; the browser and API share the same origin, with requests proxied under `/api/`.

The Nginx container listens on HTTP for a small deployment. Put it behind a TLS terminator in production; the refresh-token cookie is marked `Secure`.

Replace the example JWT, PostgreSQL, and Redis credentials before deploying publicly.
