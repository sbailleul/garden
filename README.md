# Garden Planner API

A REST API written in Rust (Actix-web) that computes the optimal planting layout for a vegetable garden based on plot dimensions, season, sun exposure, soil type, region, skill level, companion planting rules, an existing layout, and non-plantable zones (paths, alleys, obstacles).

---

## Features

- **HATEOAS** — every response includes a `_links` object (HAL convention) with hyperlinks to related resources
- **Grid-based layout optimisation** — greedy placement algorithm that maximises companion planting scores (30 cm per cell)
- **Companion planting** — `+2` per good-companion neighbour, `−3` per bad-companion neighbour
- **~40 vegetables** in an in-memory catalogue with full metadata (seasons, soil types, sun exposure, region, spacing, companions, beginner-friendliness)
- **Blocked cells** — mark paths, alleys or obstacles as non-plantable; they are preserved in the response
- **Existing layout support** — pre-place vegetables before optimisation; conflicts with blocked zones emit warnings
- **Filtering** — by season, sun, soil, region and skill level (`Beginner` / `Expert`)
- **Preference ordering** — preferred vegetables (by id) are placed first
- **Warnings** — surfaced when constraints exclude all candidates or cells cannot be filled
- **Pure in-memory** — no database required

---

## Project Structure

```
src/
  lib.rs              # library crate root
  main.rs             # binary entry point — binds to 0.0.0.0:8080
  models/
    vegetable.rs      # Vegetable struct + enums: Season, SoilType, SunExposure, Region, Category
    garden.rs         # GardenGrid, Cell (vegetable + blocked flag), PlacedVegetable
    request.rs        # PlanRequest, PlanResponse, PlannedCell, Level, CompanionsResponse, Link DTOs
  data/
    vegetables.rs     # in-memory vegetable database (~40 entries)
  logic/
    filter.rs         # filter by season / sun / soil / region / level, sort by preference
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
  plan/               # Bruno requests for the plan endpoint
```

---

## API Endpoints

All responses include a top-level envelope with the following fields:

| Field | Description |
|---|---|
| `payload` | Domain data for this endpoint |
| `errors` | Array of error/warning strings (usually empty on success) |
| `_links` | HAL HATEOAS links — `href` + `method` per relation |

Endpoints that return a list use the **paginated envelope** which adds:

| Field | Description |
|---|---|
| `pagination.page` | Current page (1-based) |
| `pagination.perPage` | Items per page |
| `pagination.total` | Total number of items |
| `pagination.totalPages` | Total number of pages |

All responses include a `_links` object following the [HAL](https://stateless.co/hal_spec/hal_spec.html) convention. Each link has an `href` field and a `method` field indicating the HTTP method to use.

### `GET /api/vegetables`

Returns the full vegetable catalogue. Response is a paginated envelope where each item includes a `payload` object with the vegetable fields, plus per-item `_links.self` and `_links.companions`.

**Response:**
```json
{
  "payload": [
    {
      "payload": { "id": "tomato", "name": "Tomato", "..." },
      "errors": [],
      "_links": {
        "self":       { "href": "/api/vegetables/tomato",            "method": "GET" },
        "companions": { "href": "/api/vegetables/tomato/companions", "method": "GET" }
      }
    }
  ],
  "errors": [],
  "_links": { "self": { "href": "/api/vegetables", "method": "GET" } },
  "pagination": { "page": 1, "perPage": 42, "total": 42, "totalPages": 1 }
}
```

---

### `GET /api/vegetables/{id}`

Returns a single vegetable by id.

**Response:**
```json
{
  "payload": {
    "id": "tomato",
    "name": "Tomato"
  },
  "errors": [],
  "_links": {
    "self":       { "href": "/api/vegetables/tomato",             "method": "GET" },
    "companions": { "href": "/api/vegetables/tomato/companions", "method": "GET" },
    "collection": { "href": "/api/vegetables",                   "method": "GET" }
  }
}
```

Returns `404` with `{ "error": "..." }` when the id is unknown.

---

### `GET /api/vegetables/{id}/companions`

Returns the good and bad companions for a given vegetable id.

**Response:**
```json
{
  "payload": {
    "id": "tomato",
    "name": "Tomato",
    "good": [{ "id": "basil", "name": "Basil" }],
    "bad":  [{ "id": "fennel", "name": "Fennel" }]
  },
  "errors": [],
  "_links": {
    "self":      { "href": "/api/vegetables/tomato/companions", "method": "GET" },
    "vegetable": { "href": "/api/vegetables/tomato",           "method": "GET" }
  }
}
```

Returns `404` with `{ "error": "..." }` when the id is unknown.

---

### `POST /api/plan`

Computes the optimal garden layout.

**Request body:**
```json
{
  "widthM": 3.0,
  "lengthM": 2.0,
  "season": "Summer",
  "sun": "FullSun",
  "soil": "Loamy",
  "region": "Temperate",
  "level": "Beginner",
  "preferences": ["tomato", "basil"],
  "existingLayout": [
    ["tomato", null, null, null, null, null, null, null, null, null],
    [null,     null, null, null, null, null, null, null, null, null]
  ],
  "blockedCells": [
    [false, false, false, false, false, false, false, false, false, false],
    [true,  true,  true,  true,  true,  true,  true,  true,  true,  true]
  ]
}
```

Required fields: `widthM`, `lengthM`, `season`. All others are optional.

| Field | Type | Description |
|---|---|---|
| `widthM` | `float` | Garden width in metres (> 0) |
| `lengthM` | `float` | Garden length in metres (> 0) |
| `season` | `Season` | Planting season |
| `sun` | `SunExposure?` | Sun exposure filter |
| `soil` | `SoilType?` | Soil type filter |
| `region` | `Region?` | Climate region filter |
| `level` | `Level?` | Skill level filter |
| `preferences` | `string[]?` | Vegetable ids to prioritise |
| `existingLayout` | `(string\|null)[][]?` | Pre-placed vegetables (grid of ids or null) |
| `blockedCells` | `bool[][]?` | Non-plantable cells — paths, alleys, obstacles |

**Enums:**

| Type | Values |
|---|---|
| `Season` | `Spring` `Summer` `Autumn` `Winter` |
| `SunExposure` | `FullSun` `PartialShade` `Shade` |
| `SoilType` | `Clay` `Sandy` `Loamy` `Chalky` `Humus` |
| `Region` | `Temperate` `Mediterranean` `Oceanic` `Continental` `Mountain` |
| `Level` | `Beginner` `Expert` |

**Response:**
```json
{
  "payload": {
    "rows": 7,
    "cols": 10,
    "score": 14,
    "warnings": [],
    "grid": [
      [
        { "id": "tomato", "name": "Tomato", "reason": "First placed (fruit, beginner-friendly) ", "blocked": false },
        { "id": null,     "name": null,     "reason": null, "blocked": true }
      ]
    ]
  },
  "errors": [],
  "_links": {
    "self":       { "href": "/api/plan",         "method": "POST" },
    "vegetables": { "href": "/api/vegetables",   "method": "GET" }
  }
}
```

Each `PlannedCell` carries:
- `id` / `name` / `reason` — `null` when the cell is empty or blocked
- `blocked` — `true` when the cell is a non-plantable zone

Returns `400` with `{ "error": "..." }` for invalid dimensions or malformed JSON.

---

## Placement Algorithm

1. Validate dimensions (must be strictly positive).
2. Compute grid size: `ceil(metres × 100 / 30)` cells per axis.
3. Mark blocked cells from `blocked_cells`.
4. Pre-fill cells from `existing_layout`; conflicting cells (blocked + vegetable) emit a warning and the vegetable is skipped.
5. For each candidate vegetable (preferences first, then alphabetical):
   - Skip if already placed.
   - Find the free, unblocked cell with the highest companion score against already-placed neighbours.
   - `score = Σ(+2 per good neighbour) + Σ(−3 per bad neighbour)`
6. Attach a human-readable reason to every placed cell.
7. Return the grid, cumulative score, and any warnings.

---

## Running Locally

```bash
# After cloning — activate the shared git hooks (one-time)
git config core.hooksPath .githooks

# Build & run
cargo run
# → server listening on http://localhost:8080

# Run all tests
cargo test
```

The pre-commit hook (`.githooks/pre-commit`) runs `cargo fmt --check`, `cargo build`, and `cargo test` before every commit and aborts on any failure.

---

## Bruno API Collection

The `bruno/` directory contains a [Bruno](https://www.usebruno.com/) collection covering all endpoints with assertions and Chai.js tests. It must always reflect the current state of the API.

```bash
# Run the collection headlessly (server must be running)
npx @usebruno/cli run bruno/ --env local
```
