# Garden Planner API

A REST API written in Rust (Actix-web) that recommends the optimal layout for a vegetable garden based on multiple parameters: plot dimensions, season, sun exposure, soil type, region, skill level, companion planting rules, and an existing layout.

---

## Features

- **Garden layout optimisation** — greedy placement algorithm that maximises companion planting scores on a grid (30 cm/cell)
- **Companion planting** — `+2` per good-companion neighbour, `-3` per bad-companion neighbour
- **~40 vegetables** in an in-memory catalogue with full metadata (seasons, soil, sun, region, spacing, companions, beginner-friendliness)
- **Existing layout support** — pre-place vegetables before optimisation
- **Warnings** — surfaced when constraints eliminate all candidates
- **Pure in-memory** — no database required

---

## Project Structure

```
src/
  lib.rs              # library crate root (re-exports all modules)
  main.rs             # binary entry point, binds to 0.0.0.0:8080
  models/
    vegetable.rs      # Vegetable struct + enums (Season, SoilType, SunExposure, Region, Category)
    garden.rs         # GardenGrid, Cell, PlacedVegetable
    request.rs        # PlanRequest, PlanResponse, CompanionsResponse DTOs
  data/
    vegetables.rs     # in-memory vegetable database (~40 entries)
  logic/
    filter.rs         # filter by season/sun/soil/region/level, sort by preference
    companion.rs      # companion_score(), is_compatible()
    planner.rs        # plan_garden() — greedy grid placement
  api/
    handlers.rs       # HTTP handlers
    routes.rs         # Actix-web route configuration
tests/
  api_integration.rs  # HTTP integration tests (actix_web::test)
  planner_e2e.rs      # realistic end-to-end scenarios
bruno/
  bruno.json          # Bruno collection manifest
  environments/
    local.bru         # baseUrl: http://localhost:8080
  vegetables/         # Bruno requests for vegetable endpoints
  plan/               # Bruno requests for plan endpoint
.github/workflows/
  ci.yml              # GitHub Actions CI (rust-tests + bruno-tests)
```

---

## API Endpoints

### `GET /api/vegetables`
Returns the full vegetable catalogue.

### `GET /api/vegetables/{id}/companions`
Returns the good and bad companions for a vegetable.

**Response:**
```json
{
  "id": "tomate",
  "name": "Tomato",
  "good": [{ "id": "basilic", "name": "Basil" }],
  "bad":  [{ "id": "fenouil", "name": "Fennel" }]
}
```

### `POST /api/plan`
Computes the optimal garden layout.

**Request body:**
```json
{
  "width_m": 3.0,
  "length_m": 2.0,
  "season": "Summer",
  "sun": "FullSun",
  "soil": "Loamy",
  "region": "Temperate",
  "level": "beginner",
  "preferences": ["tomate", "basilic"],
  "existing_layout": [
    { "row": 0, "col": 0, "vegetable_id": "tomate" }
  ]
}
```

Required fields: `width_m`, `length_m`, `season`. All others are optional.

**Enums:**
| Field | Values |
|---|---|
| `season` | `Spring` `Summer` `Autumn` `Winter` |
| `sun` | `FullSun` `PartialShade` `Shade` |
| `soil` | `Clay` `Sandy` `Loamy` `Chalky` `Humus` |
| `region` | `Temperate` `Mediterranean` `Oceanic` `Continental` `Mountain` |

**Response:**
```json
{
  "rows": 7,
  "cols": 10,
  "score": 14,
  "warnings": [],
  "grid": [[{ "id": "tomate", "name": "Tomato", "reason": "..." }, ...], ...]
}
```

---

## Running Locally

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build & run
cargo run

# Run all tests
cargo test --all -- --nocapture
```

The server binds to `http://localhost:8080`.

---

## Bruno API Collection

The `bruno/` directory contains a [Bruno](https://www.usebruno.com/) collection covering all endpoints with assertions and Chai.js tests.

```bash
# Run the collection headlessly (server must be running)
npx @usebruno/cli run bruno/ --env local
```

---

## CI/CD

GitHub Actions workflow at [.github/workflows/ci.yml](.github/workflows/ci.yml):

| Job | Steps |
|---|---|
| `rust-tests` | fmt check → clippy → build release → `cargo test --all` → upload binary |
| `bruno-tests` | download binary → start server → `bru run` → publish results summary |

---

## Placement Algorithm

1. Validate dimensions (must be strictly positive)
2. Compute grid size: `ceil(meters × 100 / 30)` cells per axis
3. Pre-fill cells from `existing_layout`
4. For each candidate vegetable (sorted: preferences first, then alphabetical):
   - Find the free cell with the highest companion score against already-placed neighbours
   - `score = Σ(+2 per good neighbour) + Σ(−3 per bad neighbour)`
   - Place the vegetable if score ≥ 0 or no other option
5. Build human-readable reasons per cell
6. Return grid, total score, and warnings
