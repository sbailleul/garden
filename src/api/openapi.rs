use utoipa::OpenApi;

use crate::models::{
    request::{
        CompanionInfo, CompanionsApiResponse, CompanionsResponse, ErrorResponse, LayoutCell, Level,
        Link, Pagination, PlanApiResponse, PlanRequest, PlanResponse, PlannedCell, PreferenceEntry,
        VegetableApiResponse, VegetableListResponse, VegetableResponse,
    },
    vegetable::{Category, Lifecycle, Region, Season, SoilType, SunExposure, Vegetable},
    Coordinate,
};

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Garden Planner API",
        description = "Companion-planting garden planner: browse a vegetable catalogue and generate optimised grid layouts based on season, soil, sun, region, skill level and planting preferences.",
        version = "1.0.0",
        license(name = "MIT"),
    ),
    paths(
        crate::api::handlers::vegetables::list_vegetables,
        crate::api::handlers::vegetables::get_vegetable,
        crate::api::handlers::vegetables::get_companions,
        crate::api::handlers::plan::post_plan,
    ),
    components(
        schemas(
            // Enums
            Season, SoilType, SunExposure, Region, Category, Lifecycle, Level,
            // Vegetable
            Vegetable, VegetableResponse,
            // Plan request
            LayoutCell, PreferenceEntry, PlanRequest,
            // Plan response
            Coordinate, PlannedCell, PlanResponse,
            // Companions
            CompanionInfo, CompanionsResponse,
            // Shared
            Link, Pagination, ErrorResponse,
            // Concrete response envelopes (via #[aliases])
            VegetableApiResponse,
            PlanApiResponse,
            CompanionsApiResponse,
            VegetableListResponse,
        )
    ),
    tags(
        (name = "vegetables", description = "Vegetable catalogue — list, detail, companion lookup"),
        (name = "plan",       description = "Garden planning — generate an optimised planting layout"),
    )
)]
pub struct ApiDoc;
