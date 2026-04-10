# Garden Planner

A monorepo containing the Garden Planner REST API, its Bruno end-to-end test collection, and a React PWA client.

---

## Repository Structure

```
api/      — Rust (Actix-web) REST API
bruno/    — Bruno API collection for end-to-end testing
```

| Directory | Description |
|---|---|
| [`api/`](api/) | Rust REST API — see [`api/README.md`](api/README.md) for full documentation |
| [`bruno/`](bruno/) | Bruno collection covering all endpoints with assertions and Chai.js tests |
| [`client/`](client/) | React 19 + TypeScript 6 PWA — vegetable catalogue browser and garden planner |

---

## Getting Started

### Prerequisites

- Rust (Edition 2021 toolchain)
- Docker + Docker Compose (for PostgreSQL)
- Node.js ≥ 20 + pnpm (for the React client)

### Start the database

```bash
docker compose up -d
# PostgreSQL 17 listens on localhost:5432
# Database: garden  User: garden  Password: garden
```

### Run the API

```bash
cd api
DATABASE_URL=postgres://garden:garden@localhost/garden cargo run
# API available at http://localhost:8080
# Migrations run automatically on startup
```

### Run the React client

```bash
cd client
pnpm install
pnpm run dev
# Development server at http://localhost:5173
```

---

## Running Tests

```bash
# Rust unit + integration tests (no database required)
cd api
cargo test

# PostgreSQL integration tests (requires a running database)
cd api
TEST_DATABASE_URL=postgres://garden:garden@localhost/garden_test cargo test -- --include-ignored

# Bruno end-to-end tests (API must be running)
cd bruno
npx @usebruno/cli run . --env local

# React UI tests (Vitest browser mode — Playwright Chromium)
cd client
pnpm run test:run
```

---

## CI

GitHub Actions (`.github/workflows/ci.yml`) runs on every push or pull request that touches `api/`, `bruno/`, or `client/`:

1. **Rust job** — `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo build --release`, `cargo test --all`
2. **Bruno job** — starts the release binary, waits for the API, then runs the full Bruno collection
3. **Client job** — `pnpm install --frozen-lockfile`, `pnpm format:check`, `pnpm lint`, `pnpm test:run` (Playwright Chromium), `pnpm build`

See [`api/README.md`](api/README.md) for full API documentation, endpoint reference, and the placement algorithm description.
