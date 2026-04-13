# Copilot Instructions

## Formatting

Run `cargo fmt` after every code modification before considering the task complete. This keeps the codebase consistently formatted and avoids pre-commit hook failures.

## Linting

Run `cargo clippy -- -D warnings` after every code modification before considering the task complete. All Clippy warnings must be resolved; warnings are treated as errors.

## Tests

Every feature must be covered by at least one automated test. This includes new endpoints, new fields, new business rules, and bug fixes. Tests live in `src/` (unit/integration via `#[cfg(test)]`) or in `tests/` (e2e tests using `actix_web::test`).

## E2E tests

When a feature adds or modifies an HTTP endpoint, add e2e tests in the appropriate file under `tests/e2e/`:

- `tests/e2e/vegetables.rs` — `GET /api/vegetables` and `GET /api/vegetables/{id}`
- `tests/e2e/companions.rs` — `GET /api/vegetables/{id}/companions`
- `tests/e2e/plan.rs` — `POST /api/plan` (all variants)
- `tests/e2e/scenarios.rs` — end-to-end planner scenarios asserting business rules

Each file uses `use crate::common::{build_app_postgres, null_layout};` and is declared as a submodule in `tests/e2e/mod.rs`. The test binary is registered in `Cargo.toml` as `[[test]] name = "e2e" path = "tests/e2e/mod.rs"`.

Shared helpers (`build_app_postgres`, `test_pool`, `null_layout`, etc.) live in `tests/common/mod.rs` and are included into the e2e binary via `#[path = "../common/mod.rs"] mod common;` at the top of `tests/e2e/mod.rs`.

## Database tests

When a feature involves database access (e.g. a new outbound adapter, a new repository method, or a new query), add database integration tests in `tests/postgres_repository.rs`. These tests run against a real PostgreSQL instance and must cover the new behaviour end-to-end (insert, fetch, edge cases). Only add database tests when the feature actually touches the persistence layer; pure domain or in-memory logic does not require them.

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

## Client UI tests

Every new client feature must be covered by at least one UI test. Tests use **Vitest** and **React Testing Library** and live in a `.test.tsx` file co-located with the component or route being tested (e.g. `src/components/vegetables/vegetable-table.test.tsx`, `src/routes/vegetables/index.test.tsx`).

- **Dumb component tests** — render the component directly with explicit props and assert the resulting output. No router or query client setup is required.
- **Smart / route component tests** — render via `RouterProvider` (with a `createMemoryHistory`) wrapped in `QueryClientProvider`. Call `queryClient.clear()` at the start of each test. Use `screen.findBy*` or `waitFor` to handle async data loading.
- **API mocking** — all HTTP calls must be intercepted by MSW. Request handlers are registered in `src/mocks/handlers.ts`. Add or extend handlers there when a new endpoint is consumed by the client.
- Use `@testing-library/user-event` for simulating user interactions (typing, clicking, etc.).

## File and directory naming (client)

All files and directories inside `client/src/` must use **kebab-case** (e.g. `vegetable-table.tsx`, `query-client.ts`, `plan-form.tsx`). This applies to every new file or directory created under that path.

## Generated files (client)

`src/routeTree.gen.ts` is auto-generated by TanStack Router. Never read, edit, or create this file. It is regenerated automatically whenever routes change.

## Smart / Dumb Component Pattern (client)

All React components in `client/src/` must follow the **smart/dumb** (container/presentational) pattern:

- **Dumb (presentational) components** — live in `src/components/`. They receive all data and callbacks via props, contain no data-fetching, no `useSuspenseQuery`, no `useMutation`, and no router hooks. They are pure UI: given the same props they render the same output.

- **Smart (container) components** — are the route components in `src/routes/`. They own data-fetching (`useSuspenseQuery`, `useMutation`), read route params and search params, and pass the resulting data down to dumb components as props.

Rules:
- A component in `src/components/` must never import from `@tanstack/react-query` or `@tanstack/react-router` (except `Link`, which is a pure UI primitive).
- A route component in `src/routes/` should delegate all rendering to one or more presentational components; it must not contain raw JSX beyond a top-level wrapper.
- Shared UI primitives (buttons, inputs, badges, etc.) stay in `src/components/ui/`. Feature-level dumb components (e.g. `VegetableTable`, `PlanGrid`, `CompanionList`) go in `src/components/` under a feature subfolder (e.g. `src/components/vegetables/`, `src/components/plan/`).


