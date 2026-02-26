# Copilot Instructions

## Tests

Every feature must be covered by at least one automated test. This includes new endpoints, new fields, new business rules, and bug fixes. Tests live in `src/` (unit/integration via `#[cfg(test)]`) or in `tests/` (integration tests using `actix_web::test`).

## Bruno collection

The Bruno collection in `bruno/` must always reflect the current state of the API. Any time an endpoint is added, modified or removed — request bodies, query parameters, assertions and test scripts must be updated accordingly.

## Documentation

`README.md` must always reflect the current state of the project. Any time a feature is added, modified or removed — endpoints, request/response fields, enums, and the placement algorithm description must be kept in sync.

## API payload naming

All JSON field names in API request and response payloads must use **camelCase** (e.g. `widthM`, `existingLayout`, `blockedCells`, `latinName`). This is enforced via `#[serde(rename_all = "camelCase")]` on every request/response struct. Enums keep their existing `PascalCase` serialisation.

## HATEOAS

Every API response must include a `_links` object following the HAL convention. Each link is an object with an `href` field and a `method` field indicating the HTTP method to use. Required links per endpoint:
- `GET /api/vegetables` — each item: `self` → `/api/vegetables/{id}`, `companions` → `/api/vegetables/{id}/companions`
- `GET /api/vegetables/{id}` — `self`, `companions`, `collection` → `/api/vegetables`
- `GET /api/vegetables/{id}/companions` — `self`, `vegetable` → `/api/vegetables/{id}`
- `POST /api/plan` — `self` → `/api/plan`, `vegetables` → `/api/vegetables`

The `_links` key is exempt from camelCase renaming and must be serialised literally as `_links` using `#[serde(rename = "_links")]`.
