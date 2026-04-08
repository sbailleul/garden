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

```bash
# Activate shared git hooks (one-time, after cloning)
git config core.hooksPath .githooks

# Build and run the API
cd api
cargo run

# Run the React client (in another terminal)
cd client
pnpm install
pnpm run dev
# → development server at http://localhost:5173
# → server listening on http://localhost:8080
```

---

## Running Tests

```bash
# Rust unit + integration tests
cd api
cargo test

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
