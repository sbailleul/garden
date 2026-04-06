# Garden Planner

A monorepo containing the Garden Planner REST API and its Bruno end-to-end test collection.

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

---

## Getting Started

```bash
# Activate shared git hooks (one-time, after cloning)
git config core.hooksPath .githooks

# Build and run the API
cd api
cargo run
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
```

---

## CI

GitHub Actions (`.github/workflows/ci.yml`) runs on every push or pull request that touches `api/` or `bruno/`:

1. **Rust job** — `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo build --release`, `cargo test --all`
2. **Bruno job** — starts the release binary, waits for the API, then runs the full Bruno collection

See [`api/README.md`](api/README.md) for full API documentation, endpoint reference, and the placement algorithm description.
