# Garden Planner API

A REST API written in Rust (Actix-web) that computes the optimal planting layout for a vegetable garden based on plot dimensions, a date range, sun exposure, soil type, region, skill level, companion planting rules, an existing layout, and non-plantable zones (paths, alleys, obstacles). The planner simulates the garden week by week: harvested plants free their cells for new plantings, and the API returns one layout snapshot per 7-day period.

---

## Features

- **HATEOAS** — every response includes a `_links` object (HAL convention) with hyperlinks to related resources
- **Grid-based layout optimisation** — greedy placement algorithm that maximises companion planting scores (30 cm per cell)
- **Companion planting** — `+2` per good-companion neighbour, `−3` per bad-companion neighbour
- **~40 vegetables** in an in-memory catalogue with full metadata (per-region sowing/planting calendars, soil types, sun exposure, spacing, days to harvest, lifecycle, companions, beginner-friendliness)
- **Blocked cells** — mark paths, alleys or obstacles as non-plantable; they are preserved in the response
- **Existing layout support** — pre-place vegetables before optimisation; conflicts with blocked zones emit warnings
- **Date-range planning** — optionally provide a `period` with `start` and `end`; the planner simulates the garden week by week, returning one `WeeklyPlan` snapshot per 7-day period (consecutive identical layouts are merged, with `weekCount` tracking how many weeks were combined). When omitted, defaults to the current Monday-to-Sunday week
- **Harvest simulation** — plants are removed when their `daysToHarvest` has elapsed, freeing cells for new plantings in subsequent weeks
- **Filtering** — by calendar month (checked against each vegetable's per-region sowing/planting windows), sun, soil, region and skill level (`Beginner` / `Expert`)
- **Preference ordering** — preferred vegetables (by id) are placed first
- **Warnings** — surfaced when constraints exclude all candidates or cells cannot be filled
- **Pure in-memory** — no database required

---

## Project Structure

The project follows **port-adapter (hexagonal) architecture** with four layers. Dependency direction: `adapters` → `application` → `domain`.

```
src/
  lib.rs                      # library crate root
  main.rs                     # binary — wires adapters and binds to 0.0.0.0:8080
  domain/
    models/
      vegetable.rs            # Vegetable struct + enums: CalendarWindow, RegionCalendar, SoilType, SunExposure, Region, Category, Lifecycle
      garden.rs               # GardenGrid, Cell (vegetable + blocked flag), PlacedVegetable
      request.rs              # PlanRequest, LayoutCell, Level DTOs
      response.rs             # PlanResponse, PlannedCell, WeeklyPlan, VegetableResponse, CompanionsResponse
    services/
      filter.rs               # filter by calendar month / sun / soil / region / level, sort by preference
      companion.rs            # companion_score(), is_compatible()
      planner.rs              # plan_garden() — greedy grid placement
      placement.rs            # find_best_block(), fill_block(), place_candidates()
      grid.rs                 # initialize_grid(), validate_layout()
      allocation.rs           # compute_explicit_allocation(), build_placement_queue()
      schedule.rs             # weeks_for_period(), generate_weeks()
      response.rs             # build_weekly_plan(), build_grid_cells(), merge_consecutive_plans()
  application/
    ports/
      vegetable_repository.rs # VegetableRepository trait (outbound port)
    use_cases/
      plan_garden.rs          # PlanGardenUseCase — fetches data, calls domain, returns PlanResponse
      vegetables.rs           # ListVegetablesUseCase, GetVegetableUseCase, GetCompanionsUseCase
  adapters/
    inbound/
      http/
        handlers/             # Actix-web HTTP handlers — instantiate use cases and delegate
        routes.rs             # route configuration
        openapi.rs            # Utoipa OpenAPI schema
    outbound/
      memory/
        vegetable_repository.rs  # InMemoryVegetableRepository (implements application::ports::VegetableRepository)
tests/
  api_integration.rs          # HTTP integration tests (actix_web::test)
  planner_e2e.rs              # realistic end-to-end scenarios
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
  "period": {"start": "2025-06-02", "end": "2025-08-31"},
  "sun": "FullSun",
  "soil": "Loamy",
  "region": "Temperate",
  "level": "Beginner",
  "preferences": [
    { "id": "tomato", "quantity": 3 },
    { "id": "basil" }
  ],
  "sown": {
    "tomato": [{"sowingDate": "2025-03-15", "seedsSown": 10}],
    "pepper": [{"sowingDate": "2025-02-20", "seedsSown": 6}, {"seedsSown": 4}]
  },
  "layout": [
    [
      { "type": "SelfContained", "id": "tomato", "plantedDate": "2025-05-01" },
      { "type": "Empty" },
      { "type": "Empty" }
    ],
    [
      { "type": "Empty" },
      { "type": "Blocked" },
      { "type": "Empty" }
    ]
  ]
}
```

Required fields: `layout`, `region`. All others are optional.

The `layout` field is a 2-D array that simultaneously defines grid dimensions and cell state.
Each cell is a JSON object with a `type` discriminator:

| Cell value | Meaning |
|---|---|
| `{"type": "Empty"}` | Free cell — plantable and empty |
| `{"type": "SelfContained", "id": "vegetable-id"}` | Pre-placed vegetable that fits in one cell (preserved in output) |
| `{"type": "SelfContained", "id": "vegetable-id", "plantedDate": "2025-05-01"}` | Same, with a planting date used for harvest scheduling and `estimatedHarvestDate` |
| `{"type": "Overflowing", "id": "vegetable-id"}` | Pre-placed vegetable that spans multiple cells (anchor cell) |
| `{"type": "Overflowing", "id": "vegetable-id", "plantedDate": "2025-05-01"}` | Same, with a planting date used for harvest scheduling and `estimatedHarvestDate` |
| `{"type": "Blocked"}` | Blocked cell — non-plantable zone (path, alley, obstacle) |

Grid dimensions are inferred directly from the array: `rows = layout.length`, `cols = layout[0].length`.

| Field | Type | Description |
|---|---|---|
| `period` | `{ start: string, end: string }?` | Planning period — both dates in ISO 8601 format. When omitted, defaults to the current Monday-to-Sunday week. If the dates do not fall on Mon/Sun boundaries they are automatically snapped outward. |
| `layout` | `LayoutCell[][]` | Grid encoding size, blocked zones, and pre-placed vegetables |
| `sun` | `SunExposure?` | Sun exposure filter |
| `soil` | `SoilType?` | Soil type filter |
| `region` | `Region` | Climate region (required) |
| `level` | `Level?` | Skill level filter |
| `preferences` | `{ id: string, quantity?: number }[]?` | Vegetables to prioritise; optional `quantity` sets the desired number of **plants** (placements) — each plant may occupy more than one cell |
| `exclusions` | `string[]?` | Vegetable IDs to exclude from planning — these will never be auto-placed regardless of other filters. Pre-placed cells in `layout` are not affected. |
| `sown` | `{ [id: string]: { sowingDate?: string, seedsSown: number }[] }?` | Vegetables already sown from seed, keyed by vegetable id — each entry is a list of sowing batches with an optional date and a seed count |

**Enums:**

| Type | Values |
|---|---|
| `SunExposure` | `FullSun` `PartialShade` `Shade` |
| `SoilType` | `Clay` `Sandy` `Loamy` `Chalky` `Humus` |
| `Region` | `Temperate` `Mediterranean` `Oceanic` `Continental` `Mountain` |
| `Lifecycle` | `Annual` `Biennial` `Perennial` |
| `Level` | `Beginner` `Expert` |

**Response:**
```json
{
  "payload": {
    "rows": 7,
    "cols": 10,
    "warnings": [],
    "weeks": [
      {
        "period": { "start": "2025-06-01", "end": "2025-06-07" },
        "weekCount": 1,
        "score": 14,
        "sowingTasks": [
          { "id": "tomato", "name": "Tomato", "targetWeekStart": "2025-07-14" }
        ],
        "grid": [
          [
            { "id": "tomato", "name": "Tomato", "reason": "...", "plantsPerCell": 1, "widthCells": 2, "lengthCells": 2, "blocked": false },
            { "coveredBy": { "row": 0, "col": 0 }, "blocked": false }
          ]
        ]
      }
    ]
  },
  "errors": [],
  "_links": {
    "self":       { "href": "/api/plan",         "method": "POST" },
    "vegetables": { "href": "/api/vegetables",   "method": "GET" }
  }
}
```

The `weeks` array contains one entry per run of consecutive 7-day periods that produced the same garden layout. Adjacent weeks with identical grids are merged into a single `WeeklyPlan`; `weekCount` tracks how many original 7-day periods were combined. When `period` is omitted the current week is used. Each `WeeklyPlan` has:

| Field | Description |
|---|---|
| `period` | Object with `start` and `end` (ISO 8601) — the Monday–Sunday range this snapshot covers (spans multiple weeks when entries are merged) |
| `weekCount` | Number of consecutive 7-day periods that produced this identical layout |
| `score` | Cumulative companion-planting score across all merged weeks |
| `sowingTasks` | Vegetables to sow during this week so they will be ready to transplant in a later planning week. Each entry: `{ id, name, targetWeekStart }` where `targetWeekStart` is the start of the target transplanting week |
| `grid` | 2-D array of `PlannedCell` objects (same structure as before) |

Each `PlannedCell` carries:
- `id` / `name` / `reason` / `plantsPerCell` / `widthCells` / `lengthCells` — present **only on the anchor cell** (top-left of the block). `null` / omitted on continuation and empty cells.
- `coveredBy: { row, col }` — present **only on continuation cells** of a multi-cell block; points to the anchor cell (0-based row/col indices). Omitted on anchor and empty cells.
- `blocked` — `true` when the cell is a non-plantable zone

For a tomato (60 cm, span 2) placed at row 0, col 0 on a 4×4 grid:

| Cell | `id` | `widthCells` | `coveredBy` |
|---|---|---|---|
| [0][0] | `"tomato"` | `2` | — |
| [0][1] | — | — | `{row:0, col:0}` |
| [1][0] | — | — | `{row:0, col:0}` |
| [1][1] | — | — | `{row:0, col:0}` |

Returns `400` with `{ "error": "..." }` for an empty `layout` or malformed JSON.

---

## Placement Algorithm

```mermaid
flowchart TD
    A([POST /api/plan]) --> B[Validate layout<br/>non-empty rows & cols]
    B -->|invalid| ERR([400 Bad Request])
    B -->|valid| C[Pre-fill grid<br/>from layout cells]
    C --> D{All free cells<br/>already occupied?}
    D -->|yes| WARN[Emit 'fully occupied'<br/>warning]
    WARN --> RESP([Return response])
    D -->|no| E[Filter vegetable DB<br/>season · sun · soil · region · level]
    E --> F[Sort candidates<br/>preferences first in declared order<br/>then by French consumption rank]
    F --> G[compute_allocation<br/>Pass 1: honour explicit quantities<br/>Pass 2: split remainder evenly<br/>round-robin extras to top candidates]
    G --> H[Expand candidate list<br/>repeat each vegetable<br/>allocation times]
    H --> I{More candidates<br/>to place?}
    I -->|no| J{Empty cells<br/>remaining?}
    I -->|yes| K[For each free span×span block<br/>compute companion score on perimeter<br/>+2 good neighbour · −3 bad neighbour]
    K --> L[Fill all span×span cells<br/>with same vegetable<br/>widthCells = lengthCells = span]
    L --> M{Count reached<br/>allocation?}
    M -->|no| I
    M -->|yes| I
    J -->|yes| N[Emit 'N empty cells'<br/>warning]
    J -->|no| RESP
    N --> RESP
    RESP --> O([200 OK<br/>weeks · rows · cols · warnings · _links])
```

1. **Validate** — `layout` must have at least one non-empty row; returns `400` otherwise.
2. **Pre-fill** — blocked cells (`{"type": "Blocked"}`) and pre-placed vegetables (`{"type": "SelfContained","id":"..."}` / `{"type": "Overflowing","id":"..."}`) are applied from the `layout` array. Unknown vegetable IDs emit a warning and are skipped.
3. **Early exit** — if every non-blocked cell is already occupied, return immediately with a warning.
4. **Filter** — the vegetable catalogue is narrowed by the week's `period.start` month, checked against each vegetable's per-region `sowing` and `planting` windows (`CalendarWindow.outdoor` / `CalendarWindow.indoor`), along with `sun`, `soil`, `region`, and `level`. Only vegetables that have an active month in the matching region are considered.
5. **Sort** — preferred vegetables appear first (in their declared order); remaining candidates are ordered by French household consumption rank (tomato → maïs); unknown IDs sort last.
6. **Phase 1 — Explicit placement** — vegetables with an explicit `quantity` preference are placed first, in preference order, each guaranteed a minimum of `quantity` plants.
   - `quantity` is a **plant count** (not a cell count); a tomato (`quantity: 2`) with 60 cm spacing (span 2 × 2 = 4 cells) reserves 8 cells.
7. **Phase 2 — Iterative fill** — after explicit preferences, all candidates (in priority order) are tried repeatedly with no per-vegetable cap until every plantable cell is occupied or no candidate can place anywhere:
   - Each pass iterates all candidates; for each one the best available `span × span` block (by companion score) is placed.
   - Passes repeat until a full pass yields zero new placements.
   - This ensures cells that were left vacant because a large-span plant could not find a free block are filled by smaller alternatives.
8. **Score** — every placement adds `Σ(+2 per good neighbour) + Σ(-3 per bad neighbour)` on the block perimeter to the cumulative companion score.
8. **Warn** — any remaining empty (non-blocked) cells produce an `"N empty cell(s)"` warning.
9. **Harvest** — before each subsequent week, cells whose plant's harvest deadline (`plantedWeek + ⌈daysToHarvest / 7⌉`) has been reached are cleared, making them available for new plantings.
10. **Repeat** — steps 4–8 are re-run for every week in the planning period; the calendar filter adapts to the new week's month.
11. **Return** — the `weeks` array (consecutive identical layouts merged, one `WeeklyPlan` per unique layout run), grid dimensions, warnings, and `_links`.

---

## Running Locally

```bash
# After cloning — activate the shared git hooks (one-time)
git config core.hooksPath .githooks

# Build & run
cd api
cargo run
# → server listening on http://localhost:8080

# Run all tests
cargo test
```

The pre-commit hook (`.githooks/pre-commit`) runs `cargo fmt --check`, `cargo build`, and `cargo test` before every commit and aborts on any failure.

---

## Bruno API Collection

The `bruno/` directory (at the repository root) contains a [Bruno](https://www.usebruno.com/) collection covering all endpoints with assertions and Chai.js tests. It must always reflect the current state of the API.

```bash
# Run the collection headlessly (server must be running)
cd bruno && npx @usebruno/cli run . --env local
```
