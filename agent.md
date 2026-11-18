# Agent guide

## Project

Postix is an MVP social publishing app between Twitter and Medium. It is a Rust workspace with a Yew/WebAssembly client, an Actix Web API, PostgreSQL, Redis, and a small shared-contract crate.

Workspace members:

- `client/` — Yew single-page application.
- `server/` — Actix Web API and persistence.
- `core/` — request/response contracts shared by client and server.

`core/` is an independent crate in the workspace, not a module or layer inside `server/`.

## Architecture

### Server: DDD

Keep dependencies flowing inward:

- `server/src/domain/` — entities, domain rules, and repository interfaces. Do not put SQL, HTTP, or framework details here.
- `server/src/application/` — use cases and orchestration through domain repository interfaces.
- `server/src/infrastructure/` — PostgreSQL/Redis adapters, migrations, auth middleware, and security implementations.
- `server/src/presentation/` — routes, handlers, extractors, DTO mapping, and HTTP error conversion.
- `server/migrations/` — forward-only database schema changes. Add a migration instead of editing an already-applied migration.

Do not bypass application services from handlers or access infrastructure repositories directly from domain code.

### Client: FSD

Keep UI and behavior in the appropriate layer:

- `client/src/app/` — app setup, router, and layout.
- `client/src/pages/` — route-level composition; keep reusable business logic out of pages.
- `client/src/widgets/` — larger reusable page sections, such as the post feed and header.
- `client/src/features/` — user actions and flows, such as authentication and creating a post.
- `client/src/entities/` — post/user models, API access, and entity UI.
- `client/src/shared/` — generic UI and cross-cutting primitives.

Prefer one-way imports from higher-level slices to lower-level layers. Do not make `shared` depend on `features`, `widgets`, or `pages`.
Import a slice through its public API at the slice root rather than reaching into another slice's internal files. Keep each slice's exports in its `mod.rs`.

## Product direction

- Use Twitter as the model for the social experience: make the home timeline the primary screen, keep browsing and publishing straightforward, and present posts as items in a shared feed.
- Use Medium as the model for authoring: provide a dedicated writing flow with a title, long-form body, tags, and a clear publish action. Do not put the editor on the feed page.
- Treat these as product directions, not a requirement to clone every feature from either service. Implement only MVP interactions supported by the existing API or explicitly added to its contract.
- Do not imply that follows, replies, or other social interactions work until their client and API flows are implemented and tested.

## Current MVP behavior and constraints

- Unauthenticated visitors may access About, sign-in, and sign-up only.
- Protected routes wait for session validation, then redirect unauthenticated users to sign-in.
- The client refreshes the access token on startup and retries a protected request once after `401`. Invalid refresh sessions clear local auth state and redirect to sign-in.
- Home is the feed. Post creation is a separate page at `/posts/new`; post details use `/posts/:id`.
- The authenticated profile provides published-post and draft views; drafts and deleted posts are private.
- Keep sign-out on the profile page only.
- The writing form accepts a title, plain-text body, and tags. Authors can publish immediately or save a draft; confirm before committing publish, draft, or sign-out actions.
- Feed cards link to the full post, and previews show at most the first 150 characters of the body.
- Post details show a 1–5 star rating, comments, and per-comment Agree/Disagree reactions. Keep comment reactions distinct from numeric post ratings.
- Search filters the posts currently loaded by the client; it is not a full server-side search.
- Blueprint blocks share the app's blue/green/red action palette and consistent hover treatment; preserve the shared styles when changing pages.
- Drafts and soft-deleted posts must not be exposed as public posts.

Preserve these behaviors when changing routing, auth, or post flows. Add a server endpoint only when the MVP needs behavior that the existing API cannot provide.

## Development

Set up local dependencies and run the services:

```bash
cp .env.example .env
docker compose up -d postgres redis
cargo run -p server
cd client && trunk serve
```

Build the client from its package directory:

```bash
cd client && trunk build --release
```

Run the primary checks from the workspace root:

```bash
cargo fmt --all -- --check
cargo test --workspace
```

## Change and test expectations

- Make small, focused changes and follow existing Rust/Yew patterns.
- Keep validation and authorization in the domain/application layers as appropriate; handlers should translate HTTP input and output.
- Treat all client input and API responses as untrusted. Never render post content as raw HTML.
- Do not log access tokens, refresh tokens, passwords, or other credentials. Keep secrets in environment configuration, not source.
- Add unit tests for new domain rules, transformations, and validation.
- Add Actix integration tests for new or changed API behavior, including authentication and response contracts where relevant.
- Add client tests for routing, API contracts, and state/refresh behavior. Run the WebAssembly production build when client code changes.
- Keep tests deterministic and avoid requiring a live PostgreSQL or Redis instance unless the test is explicitly an environment-backed integration test.
- Add each database change as a forward-only migration and update `server/src/infrastructure/schema.dbml` in the same change.
- Run the narrowest relevant tests while iterating, then run `cargo test --workspace` and formatting checks before finishing.
- Update this file or `README.md` if project commands, architecture, or MVP behavior materially changes.
