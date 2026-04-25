use utoipa::OpenApi;

use crate::adapters::inbound::http::hateoas::{
    CompanionsApiResponse, ErrorResponse, Link, Pagination, PlanApiResponse, VarietiesApiResponse,
    VarietyApiResponse, VegetableApiResponse, VegetablesApiResponse,
};
use crate::domain::models::{
    request::{LayoutCell, Level, Period, PlanRequest, PreferenceEntry, SowingRecord},
    response::{
        CompanionInfo, CompanionsResponse, PlanResponse, PlannedCell, SowingTask, WeeklyPlan,
    },
    variety::{
        CalendarWindow, Category, Lifecycle, Month, Region, RegionCalendar, SoilType, SunExposure,
        Variety,
    },
    vegetable::Vegetable,
    Coordinate,
};

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Garden Planner API",
        description = "Companion-planting garden planner: browse a variety catalogue and generate optimised grid layouts based on season, soil, sun, region, skill level and planting preferences.",
        version = "1.0.0",
        license(name = "MIT"),
    ),
    paths(
        crate::adapters::inbound::http::handlers::varieties::list_varieties,
        crate::adapters::inbound::http::handlers::varieties::get_variety,
        crate::adapters::inbound::http::handlers::vegetables::list_vegetables,
        crate::adapters::inbound::http::handlers::vegetables::get_vegetable,
        crate::adapters::inbound::http::handlers::vegetables::get_companions,
        crate::adapters::inbound::http::handlers::vegetables::get_varieties_by_vegetable,
        crate::adapters::inbound::http::handlers::plan::post_plan,
    ),
    components(
        schemas(
            // Enums
            SoilType, SunExposure, Region, Category, Lifecycle, Level, Month,
            // Variety calendar
            CalendarWindow, RegionCalendar,
            // Variety
            Variety,
            // Vegetable
            Vegetable,
            // Plan request
            LayoutCell, PreferenceEntry, Period, SowingRecord, PlanRequest,
            // Plan response
            Coordinate, PlannedCell, SowingTask, WeeklyPlan, PlanResponse,
            // Companions
            CompanionInfo, CompanionsResponse,
            // Shared
            Link, Pagination, ErrorResponse,
            // Concrete response envelopes (via #[aliases])
            VarietyApiResponse,
            VarietiesApiResponse,
            VegetableApiResponse,
            VegetablesApiResponse,
            PlanApiResponse,
            CompanionsApiResponse,
        )
    ),
    tags(
        (name = "varieties", description = "Variety catalogue — list, detail, companion lookup"),
        (name = "vegetables",  description = "Vegetable catalogue — group varieties by species/type"),
        (name = "plan",       description = "Garden planning — generate an optimised planting layout"),
    )
)]
pub struct ApiDoc;
