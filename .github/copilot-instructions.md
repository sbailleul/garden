# Copilot Instructions

## Formatting

Run `cargo fmt` after every code modification before considering the task complete. This keeps the codebase consistently formatted and avoids pre-commit hook failures.

## Tests

Every feature must be covered by at least one automated test. This includes new endpoints, new fields, new business rules, and bug fixes. Tests live in `src/` (unit/integration via `#[cfg(test)]`) or in `tests/` (integration tests using `actix_web::test`).

## Bruno collection

The Bruno collection in `bruno/` must always reflect the current state of the API. Any time an endpoint is added, modified or removed — request bodies, query parameters, assertions and test scripts must be updated accordingly.

## Documentation

`README.md` must always reflect the current state of the project. Any time a feature is added, modified or removed — endpoints, request/response fields, enums, and the placement algorithm description must be kept in sync.

## API payload naming

All JSON field names in API request and response payloads must use **camelCase** (e.g. `widthM`, `existingLayout`, `blockedCells`, `latinName`). This is enforced via `#[serde(rename_all = "camelCase")]` on every request/response struct. Enums keep their existing `PascalCase` serialisation.

For enum serialization:
- Enum values must be serialised in `PascalCase`.
- Enum struct fields must be serialised in `camelCase`.
- For tagged enums, the enum `type` value must be in `PascalCase`.

## Named structs over tuples

Prefer named structs over bare tuples and tuple structs when a pair or group of values has a specific meaning. For example, use `struct WeekRange { start: NaiveDate, end: NaiveDate }` rather than `(NaiveDate, NaiveDate)` or `struct WeekRange(NaiveDate, NaiveDate)`. This improves readability, enables `impl` blocks, and makes function signatures and field accesses self-documenting.

## Module visibility

Export modules only when necessary.
- Default to private modules (`mod ...`).
- Use `pub mod ...` only for modules that must be accessed from outside their parent module.
- Keep internal implementation modules private and expose only the minimal public API.

## Port-Adapter (Hexagonal) Architecture

The project must follow the **port-adapter (hexagonal) architecture** with four distinct layers:

- **`domain/`** — pure business logic with no I/O or use-case orchestration.
  - `domain/models/` — domain model structs and enums (Vegetable, GardenGrid, requests, responses, etc.).
  - `domain/services/` — domain services (placement, companion scoring, grid operations, filtering, scheduling, etc.). These modules must not import from `adapters/` or `application/`.

- **`application/`** — use-case orchestration layer that sits between the domain and the outside world.
  - `application/ports/` — outbound port traits (e.g. `VegetableRepository`). Defined here and implemented by adapters.
  - `application/use_cases/` — one struct per use case. Each use case receives an outbound port via its constructor, fetches data, calls domain services, and returns a result ready for the inbound adapter (e.g. `PlanGardenUseCase`, `ListVegetablesUseCase`).

- **`adapters/inbound/http/`** — inbound (driving) HTTP adapter built with Actix-web. Handlers receive `web::Data<Box<dyn Port + Send + Sync>>`, instantiate the matching use case, and delegate to it.

- **`adapters/outbound/memory/`** — outbound (driven) adapter. Concrete structs (e.g. `InMemoryVegetableRepository`) implement the port traits defined in `application/ports/`.

Dependency direction: `adapters` → `application` → `domain`. Neither `domain` nor `application` may import from `adapters`.

## HATEOAS

Every API response must include a `_links` object following the HAL convention. Each link is an object with an `href` field and a `method` field indicating the HTTP method to use. Required links per endpoint:
- `GET /api/vegetables` — each item: `self` → `/api/vegetables/{id}`, `companions` → `/api/vegetables/{id}/companions`
- `GET /api/vegetables/{id}` — `self`, `companions`, `collection` → `/api/vegetables`
- `GET /api/vegetables/{id}/companions` — `self`, `vegetable` → `/api/vegetables/{id}`
- `POST /api/plan` — `self` → `/api/plan`, `vegetables` → `/api/vegetables`

The `_links` key is exempt from camelCase renaming and must be serialised literally as `_links` using `#[serde(rename = "_links")]`.


