use utoipa::OpenApi;

use crate::models::{
    hateoas::{
        CompanionsApiResponse, Link, Pagination, PlanApiResponse, VegetableApiResponse,
        VegetableListResponse,
    },
    request::{
        CompanionInfo, CompanionsResponse, ErrorResponse, LayoutCell, Level, PlanRequest,
        PlanResponse, PlannedCell, PreferenceEntry, VegetableResponse, WeeklyPlan,
    },
    vegetable::{
        CalendarWindow, Category, Lifecycle, Month, Region, RegionCalendar, SoilType, SunExposure,
        Vegetable,
    },
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
            SoilType, SunExposure, Region, Category, Lifecycle, Level, Month,
            // Vegetable calendar
            CalendarWindow, RegionCalendar,
            // Vegetable
            Vegetable, VegetableResponse,
            // Plan request
            LayoutCell, PreferenceEntry, PlanRequest,
            // Plan response
            Coordinate, PlannedCell, WeeklyPlan, PlanResponse,
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
