#![allow(dead_code)]

use actix_web::{web, App};
use async_trait::async_trait;
use garden::adapters::inbound::http::routes::configure;
use garden::application::ports::vegetable_repository::{RepositoryError, VegetableRepository};
use garden::domain::models::vegetable::Month::*;
use garden::domain::models::vegetable::{
    CalendarWindow, Category, Lifecycle, Region, RegionCalendar, SoilType, SunExposure, Vegetable,
};

// ---------------------------------------------------------------------------
// In-memory test repository
// ---------------------------------------------------------------------------

pub struct TestVegetableRepository(pub Vec<Vegetable>);

#[async_trait]
impl VegetableRepository for TestVegetableRepository {
    async fn get_all(&self, _locale: &str) -> Result<Vec<Vegetable>, RepositoryError> {
        Ok(self.0.clone())
    }

    async fn get_by_id(
        &self,
        id: &str,
        _locale: &str,
    ) -> Result<Option<Vegetable>, RepositoryError> {
        Ok(self.0.iter().find(|v| v.id == id).cloned())
    }
}

// ---------------------------------------------------------------------------
// Actix-web test app builder
// ---------------------------------------------------------------------------

pub fn build_app() -> actix_web::App<
    impl actix_web::dev::ServiceFactory<
        actix_web::dev::ServiceRequest,
        Config = (),
        Response = actix_web::dev::ServiceResponse,
        Error = actix_web::Error,
        InitError = (),
    >,
> {
    let repo: Box<dyn VegetableRepository> =
        Box::new(TestVegetableRepository(get_all_vegetables()));
    App::new()
        .app_data(web::Data::new(repo))
        .configure(configure)
        .app_data(web::JsonConfig::default().error_handler(|err, _req| {
            let message = format!("{err}");
            actix_web::error::InternalError::from_response(
                err,
                actix_web::HttpResponse::BadRequest().json(serde_json::json!({ "error": message })),
            )
            .into()
        }))
}

// ---------------------------------------------------------------------------
// Helpers shared by test files
// ---------------------------------------------------------------------------

pub fn null_layout(rows: usize, cols: usize) -> serde_json::Value {
    let empty_cell = serde_json::json!({"type": "Empty"});
    let row: Vec<serde_json::Value> = vec![empty_cell; cols];
    let layout: Vec<serde_json::Value> = (0..rows)
        .map(|_| serde_json::Value::Array(row.clone()))
        .collect();
    serde_json::Value::Array(layout)
}

// ---------------------------------------------------------------------------
// Vegetable catalogue — mirrors the former InMemoryVegetableRepository data
// ---------------------------------------------------------------------------

pub fn get_all_vegetables() -> Vec<Vegetable> {
    vec![
        Vegetable {
            id: "tomato".into(),
            name: "Tomato".into(),
            latin_name: "Solanum lycopersicum".into(),
            calendars: vec![
                RegionCalendar {
                    region: Region::Temperate,
                    sowing: CalendarWindow {
                        outdoor: vec![April, May],
                        indoor: vec![February, March],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![May, June, July],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Mediterranean,
                    sowing: CalendarWindow {
                        outdoor: vec![March, April],
                        indoor: vec![January, February],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![April, May],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Oceanic,
                    sowing: CalendarWindow {
                        outdoor: vec![April, May],
                        indoor: vec![February, March],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![May, June, July],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Continental,
                    sowing: CalendarWindow {
                        outdoor: vec![April, May],
                        indoor: vec![February, March],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![May, June, July],
                        indoor: vec![],
                    },
                },
            ],
            sun_requirement: vec![SunExposure::FullSun],
            soil_types: vec![SoilType::Loamy, SoilType::Humus],
            spacing_cm: 60,
            days_to_harvest: 75,
            days_to_plant: 42,
            lifecycle: Lifecycle::Annual,
            good_companions: vec![
                "basil".into(),
                "carrot".into(),
                "parsley".into(),
                "garlic".into(),
                "onion".into(),
            ],
            bad_companions: vec!["fennel".into(), "broccoli".into(), "cabbage".into()],
            beginner_friendly: true,
            category: Category::Fruit,
        },
        Vegetable {
            id: "zucchini".into(),
            name: "Zucchini".into(),
            latin_name: "Cucurbita pepo".into(),
            calendars: vec![
                RegionCalendar {
                    region: Region::Temperate,
                    sowing: CalendarWindow {
                        outdoor: vec![April, May],
                        indoor: vec![March, April],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![May, June, July],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Mediterranean,
                    sowing: CalendarWindow {
                        outdoor: vec![March, April],
                        indoor: vec![February, March],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![April, May],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Oceanic,
                    sowing: CalendarWindow {
                        outdoor: vec![April, May],
                        indoor: vec![March, April],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![May, June, July],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Continental,
                    sowing: CalendarWindow {
                        outdoor: vec![April, May],
                        indoor: vec![March, April],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![May, June, July],
                        indoor: vec![],
                    },
                },
            ],
            sun_requirement: vec![SunExposure::FullSun],
            soil_types: vec![SoilType::Loamy, SoilType::Humus, SoilType::Clay],
            spacing_cm: 90,
            days_to_harvest: 55,
            days_to_plant: 21,
            lifecycle: Lifecycle::Annual,
            good_companions: vec![
                "green-bean".into(),
                "maïs".into(),
                "radish".into(),
                "nasturtium".into(),
            ],
            bad_companions: vec!["potato".into()],
            beginner_friendly: true,
            category: Category::Fruit,
        },
        Vegetable {
            id: "carrot".into(),
            name: "Carrot".into(),
            latin_name: "Daucus carota".into(),
            calendars: vec![
                RegionCalendar {
                    region: Region::Temperate,
                    sowing: CalendarWindow {
                        outdoor: vec![March, April, May, June, July, August],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Oceanic,
                    sowing: CalendarWindow {
                        outdoor: vec![February, March, April, May, June, July, August],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Continental,
                    sowing: CalendarWindow {
                        outdoor: vec![April, May, June, July],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Mediterranean,
                    sowing: CalendarWindow {
                        outdoor: vec![February, March, April, May, June, July, August, September],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![],
                        indoor: vec![],
                    },
                },
            ],
            sun_requirement: vec![SunExposure::FullSun, SunExposure::PartialShade],
            soil_types: vec![SoilType::Sandy, SoilType::Loamy],
            spacing_cm: 10,
            days_to_harvest: 75,
            days_to_plant: 0,
            lifecycle: Lifecycle::Annual,
            good_companions: vec![
                "tomato".into(),
                "onion".into(),
                "leek".into(),
                "lettuce".into(),
                "radish".into(),
            ],
            bad_companions: vec!["dill".into(), "fennel".into()],
            beginner_friendly: true,
            category: Category::Root,
        },
        Vegetable {
            id: "basil".into(),
            name: "Basil".into(),
            latin_name: "Ocimum basilicum".into(),
            calendars: vec![
                RegionCalendar {
                    region: Region::Mediterranean,
                    sowing: CalendarWindow {
                        outdoor: vec![April, May, June],
                        indoor: vec![February, March],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![May, June],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Temperate,
                    sowing: CalendarWindow {
                        outdoor: vec![May, June],
                        indoor: vec![March, April],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![May, June],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Oceanic,
                    sowing: CalendarWindow {
                        outdoor: vec![May, June],
                        indoor: vec![March, April],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![May, June],
                        indoor: vec![],
                    },
                },
            ],
            sun_requirement: vec![SunExposure::FullSun],
            soil_types: vec![SoilType::Loamy, SoilType::Humus],
            spacing_cm: 20,
            days_to_harvest: 30,
            days_to_plant: 21,
            lifecycle: Lifecycle::Annual,
            good_companions: vec!["tomato".into(), "pepper".into(), "asparagus".into()],
            bad_companions: vec!["sage".into(), "thyme".into()],
            beginner_friendly: true,
            category: Category::Herb,
        },
        Vegetable {
            id: "lettuce".into(),
            name: "Lettuce".into(),
            latin_name: "Lactuca sativa".into(),
            calendars: vec![
                RegionCalendar {
                    region: Region::Temperate,
                    sowing: CalendarWindow {
                        outdoor: vec![March, April, May, August, September],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![April, May, August, September],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Oceanic,
                    sowing: CalendarWindow {
                        outdoor: vec![February, March, April, May, August, September, October],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![March, April, May, August, September, October],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Continental,
                    sowing: CalendarWindow {
                        outdoor: vec![April, May, August, September],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![April, May, August, September],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Mediterranean,
                    sowing: CalendarWindow {
                        outdoor: vec![February, March, April, May, August, September, October],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![March, April, May, August, September, October],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Mountain,
                    sowing: CalendarWindow {
                        outdoor: vec![April, May, June, July, August],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![May, June, July, August],
                        indoor: vec![],
                    },
                },
            ],
            sun_requirement: vec![SunExposure::PartialShade, SunExposure::FullSun],
            soil_types: vec![SoilType::Loamy, SoilType::Humus, SoilType::Clay],
            spacing_cm: 30,
            days_to_harvest: 45,
            days_to_plant: 21,
            lifecycle: Lifecycle::Annual,
            good_companions: vec![
                "carrot".into(),
                "radish".into(),
                "strawberry".into(),
                "cucumber".into(),
            ],
            bad_companions: vec!["parsley".into(), "celery".into()],
            beginner_friendly: true,
            category: Category::Leafy,
        },
        Vegetable {
            id: "radish".into(),
            name: "Radish".into(),
            latin_name: "Raphanus sativus".into(),
            calendars: vec![
                RegionCalendar {
                    region: Region::Temperate,
                    sowing: CalendarWindow {
                        outdoor: vec![March, April, May, August, September],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Oceanic,
                    sowing: CalendarWindow {
                        outdoor: vec![February, March, April, May, August, September, October],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Continental,
                    sowing: CalendarWindow {
                        outdoor: vec![April, May, August, September],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Mediterranean,
                    sowing: CalendarWindow {
                        outdoor: vec![February, March, April, May, August, September, October],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Mountain,
                    sowing: CalendarWindow {
                        outdoor: vec![April, May, June, July, August],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![],
                        indoor: vec![],
                    },
                },
            ],
            sun_requirement: vec![SunExposure::FullSun, SunExposure::PartialShade],
            soil_types: vec![SoilType::Sandy, SoilType::Loamy],
            spacing_cm: 5,
            days_to_harvest: 25,
            days_to_plant: 0,
            lifecycle: Lifecycle::Annual,
            good_companions: vec![
                "carrot".into(),
                "lettuce".into(),
                "tomato".into(),
                "cucumber".into(),
            ],
            bad_companions: vec!["hyssop".into()],
            beginner_friendly: true,
            category: Category::Root,
        },
        Vegetable {
            id: "onion".into(),
            name: "Onion".into(),
            latin_name: "Allium cepa".into(),
            calendars: vec![
                RegionCalendar {
                    region: Region::Temperate,
                    sowing: CalendarWindow {
                        outdoor: vec![February, March, April, July, August],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![April, May],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Continental,
                    sowing: CalendarWindow {
                        outdoor: vec![March, April, July, August],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![April, May],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Oceanic,
                    sowing: CalendarWindow {
                        outdoor: vec![February, March, April, July, August, September],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![March, April, May],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Mediterranean,
                    sowing: CalendarWindow {
                        outdoor: vec![February, March, April, July, August, September, October],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![March, April, May],
                        indoor: vec![],
                    },
                },
            ],
            sun_requirement: vec![SunExposure::FullSun],
            soil_types: vec![SoilType::Loamy, SoilType::Sandy],
            spacing_cm: 10,
            days_to_harvest: 100,
            days_to_plant: 60,
            lifecycle: Lifecycle::Annual,
            good_companions: vec![
                "carrot".into(),
                "tomato".into(),
                "beet".into(),
                "lettuce".into(),
            ],
            bad_companions: vec!["green-bean".into(), "pea".into(), "garlic".into()],
            beginner_friendly: true,
            category: Category::Bulb,
        },
        Vegetable {
            id: "garlic".into(),
            name: "Garlic".into(),
            latin_name: "Allium sativum".into(),
            calendars: vec![
                RegionCalendar {
                    region: Region::Temperate,
                    sowing: CalendarWindow {
                        outdoor: vec![],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![October, November, March],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Mediterranean,
                    sowing: CalendarWindow {
                        outdoor: vec![],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![September, October, November, February, March],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Continental,
                    sowing: CalendarWindow {
                        outdoor: vec![],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![October, November, March, April],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Oceanic,
                    sowing: CalendarWindow {
                        outdoor: vec![],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![October, November, December, March],
                        indoor: vec![],
                    },
                },
            ],
            sun_requirement: vec![SunExposure::FullSun],
            soil_types: vec![SoilType::Loamy, SoilType::Sandy],
            spacing_cm: 10,
            days_to_harvest: 240,
            days_to_plant: 0,
            lifecycle: Lifecycle::Annual,
            good_companions: vec![
                "tomato".into(),
                "rose".into(),
                "strawberry".into(),
                "carrot".into(),
            ],
            bad_companions: vec!["onion".into(), "green-bean".into(), "pea".into()],
            beginner_friendly: true,
            category: Category::Bulb,
        },
        Vegetable {
            id: "leek".into(),
            name: "Leek".into(),
            latin_name: "Allium porrum".into(),
            calendars: vec![
                RegionCalendar {
                    region: Region::Temperate,
                    sowing: CalendarWindow {
                        outdoor: vec![February, March, April],
                        indoor: vec![January, February],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![May, June, July],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Oceanic,
                    sowing: CalendarWindow {
                        outdoor: vec![February, March, April],
                        indoor: vec![January, November, December],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![May, June, July],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Continental,
                    sowing: CalendarWindow {
                        outdoor: vec![March, April],
                        indoor: vec![January, February],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![May, June, July],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Mountain,
                    sowing: CalendarWindow {
                        outdoor: vec![March, April, May],
                        indoor: vec![February, March],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![June, July],
                        indoor: vec![],
                    },
                },
            ],
            sun_requirement: vec![SunExposure::FullSun],
            soil_types: vec![SoilType::Loamy, SoilType::Clay, SoilType::Humus],
            spacing_cm: 15,
            days_to_harvest: 120,
            days_to_plant: 60,
            lifecycle: Lifecycle::Biennial,
            good_companions: vec!["carrot".into(), "celery".into(), "lettuce".into()],
            bad_companions: vec!["green-bean".into(), "pea".into()],
            beginner_friendly: true,
            category: Category::Bulb,
        },
        Vegetable {
            id: "green-bean".into(),
            name: "Green Bean".into(),
            latin_name: "Phaseolus vulgaris".into(),
            calendars: vec![
                RegionCalendar {
                    region: Region::Temperate,
                    sowing: CalendarWindow {
                        outdoor: vec![April, May, June, July],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![May, June, July],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Oceanic,
                    sowing: CalendarWindow {
                        outdoor: vec![April, May, June, July],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![May, June, July],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Continental,
                    sowing: CalendarWindow {
                        outdoor: vec![May, June, July],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![May, June, July],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Mediterranean,
                    sowing: CalendarWindow {
                        outdoor: vec![March, April, May, June, July, August],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![April, May, June, July, August],
                        indoor: vec![],
                    },
                },
            ],
            sun_requirement: vec![SunExposure::FullSun],
            soil_types: vec![SoilType::Loamy, SoilType::Sandy, SoilType::Humus],
            spacing_cm: 15,
            days_to_harvest: 55,
            days_to_plant: 0,
            lifecycle: Lifecycle::Annual,
            good_companions: vec![
                "zucchini".into(),
                "maïs".into(),
                "potato".into(),
                "radish".into(),
            ],
            bad_companions: vec![
                "onion".into(),
                "garlic".into(),
                "fennel".into(),
                "leek".into(),
            ],
            beginner_friendly: true,
            category: Category::Pod,
        },
        Vegetable {
            id: "cucumber".into(),
            name: "Cucumber".into(),
            latin_name: "Cucumis sativus".into(),
            calendars: vec![
                RegionCalendar {
                    region: Region::Temperate,
                    sowing: CalendarWindow {
                        outdoor: vec![April, May],
                        indoor: vec![March, April],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![May, June],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Mediterranean,
                    sowing: CalendarWindow {
                        outdoor: vec![March, April],
                        indoor: vec![February, March],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![April, May],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Oceanic,
                    sowing: CalendarWindow {
                        outdoor: vec![April, May],
                        indoor: vec![March, April],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![May, June],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Continental,
                    sowing: CalendarWindow {
                        outdoor: vec![April, May],
                        indoor: vec![March, April],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![May, June],
                        indoor: vec![],
                    },
                },
            ],
            sun_requirement: vec![SunExposure::FullSun],
            soil_types: vec![SoilType::Loamy, SoilType::Humus],
            spacing_cm: 60,
            days_to_harvest: 55,
            days_to_plant: 21,
            lifecycle: Lifecycle::Annual,
            good_companions: vec![
                "radish".into(),
                "lettuce".into(),
                "green-bean".into(),
                "maïs".into(),
            ],
            bad_companions: vec!["tomato".into(), "potato".into(), "fennel".into()],
            beginner_friendly: true,
            category: Category::Fruit,
        },
        Vegetable {
            id: "pepper".into(),
            name: "Pepper".into(),
            latin_name: "Capsicum annuum".into(),
            calendars: vec![
                RegionCalendar {
                    region: Region::Mediterranean,
                    sowing: CalendarWindow {
                        outdoor: vec![March, April],
                        indoor: vec![January, February, March],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![April, May],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Temperate,
                    sowing: CalendarWindow {
                        outdoor: vec![April, May],
                        indoor: vec![February, March],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![May, June],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Oceanic,
                    sowing: CalendarWindow {
                        outdoor: vec![April, May],
                        indoor: vec![February, March],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![May, June],
                        indoor: vec![],
                    },
                },
            ],
            sun_requirement: vec![SunExposure::FullSun],
            soil_types: vec![SoilType::Loamy, SoilType::Humus],
            spacing_cm: 50,
            days_to_harvest: 70,
            days_to_plant: 60,
            lifecycle: Lifecycle::Annual,
            good_companions: vec!["basil".into(), "tomato".into(), "carrot".into()],
            bad_companions: vec!["fennel".into(), "broccoli".into()],
            beginner_friendly: false,
            category: Category::Fruit,
        },
        Vegetable {
            id: "pea".into(),
            name: "Pea".into(),
            latin_name: "Pisum sativum".into(),
            calendars: vec![
                RegionCalendar {
                    region: Region::Temperate,
                    sowing: CalendarWindow {
                        outdoor: vec![February, March, April, September, October],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![March, April, May],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Oceanic,
                    sowing: CalendarWindow {
                        outdoor: vec![February, March, April, September, October, November],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![March, April, May],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Continental,
                    sowing: CalendarWindow {
                        outdoor: vec![March, April, September],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![April, May],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Mountain,
                    sowing: CalendarWindow {
                        outdoor: vec![March, April, May, June],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![May, June],
                        indoor: vec![],
                    },
                },
            ],
            sun_requirement: vec![SunExposure::FullSun, SunExposure::PartialShade],
            soil_types: vec![SoilType::Loamy, SoilType::Clay, SoilType::Chalky],
            spacing_cm: 10,
            days_to_harvest: 60,
            days_to_plant: 0,
            lifecycle: Lifecycle::Annual,
            good_companions: vec![
                "carrot".into(),
                "radish".into(),
                "lettuce".into(),
                "cabbage".into(),
            ],
            bad_companions: vec!["onion".into(), "garlic".into(), "fennel".into()],
            beginner_friendly: true,
            category: Category::Pod,
        },
        Vegetable {
            id: "cabbage".into(),
            name: "Cabbage".into(),
            latin_name: "Brassica oleracea".into(),
            calendars: vec![
                RegionCalendar {
                    region: Region::Temperate,
                    sowing: CalendarWindow {
                        outdoor: vec![March, April, May, July, August],
                        indoor: vec![January, February],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![May, June, September, October],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Oceanic,
                    sowing: CalendarWindow {
                        outdoor: vec![March, April, May, July, August],
                        indoor: vec![January, February, November, December],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![April, May, June, September, October],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Continental,
                    sowing: CalendarWindow {
                        outdoor: vec![April, May, July, August],
                        indoor: vec![February, March],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![May, June, September],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Mountain,
                    sowing: CalendarWindow {
                        outdoor: vec![March, April, May, June],
                        indoor: vec![February, March, April],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![June, July, August],
                        indoor: vec![],
                    },
                },
            ],
            sun_requirement: vec![SunExposure::FullSun],
            soil_types: vec![SoilType::Loamy, SoilType::Clay, SoilType::Chalky],
            spacing_cm: 50,
            days_to_harvest: 90,
            days_to_plant: 35,
            lifecycle: Lifecycle::Annual,
            good_companions: vec!["celery".into(), "onion".into(), "pea".into()],
            bad_companions: vec!["tomato".into(), "strawberry".into(), "fennel".into()],
            beginner_friendly: false,
            category: Category::Leafy,
        },
        Vegetable {
            id: "broccoli".into(),
            name: "Broccoli".into(),
            latin_name: "Brassica oleracea var. italica".into(),
            calendars: vec![
                RegionCalendar {
                    region: Region::Temperate,
                    sowing: CalendarWindow {
                        outdoor: vec![March, April, May, July, August],
                        indoor: vec![January, February],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![May, June, September],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Oceanic,
                    sowing: CalendarWindow {
                        outdoor: vec![March, April, July, August],
                        indoor: vec![January, February],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![April, May, September, October],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Continental,
                    sowing: CalendarWindow {
                        outdoor: vec![April, May, July, August],
                        indoor: vec![February, March],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![May, June, September],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Mountain,
                    sowing: CalendarWindow {
                        outdoor: vec![April, May, June],
                        indoor: vec![February, March, April],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![June, July],
                        indoor: vec![],
                    },
                },
            ],
            sun_requirement: vec![SunExposure::FullSun],
            soil_types: vec![SoilType::Loamy, SoilType::Clay, SoilType::Chalky],
            spacing_cm: 50,
            days_to_harvest: 80,
            days_to_plant: 35,
            lifecycle: Lifecycle::Annual,
            good_companions: vec!["celery".into(), "onion".into()],
            bad_companions: vec!["tomato".into(), "pepper".into(), "strawberry".into()],
            beginner_friendly: false,
            category: Category::Leafy,
        },
        Vegetable {
            id: "parsley".into(),
            name: "Parsley".into(),
            latin_name: "Petroselinum crispum".into(),
            calendars: vec![
                RegionCalendar {
                    region: Region::Temperate,
                    sowing: CalendarWindow {
                        outdoor: vec![March, April, May, June, July, August],
                        indoor: vec![January, February],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Oceanic,
                    sowing: CalendarWindow {
                        outdoor: vec![February, March, April, May, June, July, August, September],
                        indoor: vec![January, February],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Continental,
                    sowing: CalendarWindow {
                        outdoor: vec![April, May, June, July],
                        indoor: vec![February, March],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Mediterranean,
                    sowing: CalendarWindow {
                        outdoor: vec![
                            February, March, April, May, June, July, August, September, October,
                        ],
                        indoor: vec![January],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![],
                        indoor: vec![],
                    },
                },
            ],
            sun_requirement: vec![SunExposure::FullSun, SunExposure::PartialShade],
            soil_types: vec![SoilType::Loamy, SoilType::Humus],
            spacing_cm: 20,
            days_to_harvest: 75,
            days_to_plant: 21,
            lifecycle: Lifecycle::Biennial,
            good_companions: vec!["tomato".into(), "asparagus".into(), "rose".into()],
            bad_companions: vec!["lettuce".into()],
            beginner_friendly: true,
            category: Category::Herb,
        },
        Vegetable {
            id: "thyme".into(),
            name: "Thyme".into(),
            latin_name: "Thymus vulgaris".into(),
            calendars: vec![
                RegionCalendar {
                    region: Region::Mediterranean,
                    sowing: CalendarWindow {
                        outdoor: vec![March, April, May, June, July, August, September, October],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![March, April, May, September, October],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Temperate,
                    sowing: CalendarWindow {
                        outdoor: vec![April, May, June, July, August],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![April, May, June, September],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Oceanic,
                    sowing: CalendarWindow {
                        outdoor: vec![March, April, May, June, July, August, September],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![April, May, June, September, October],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Continental,
                    sowing: CalendarWindow {
                        outdoor: vec![April, May, June, July, August],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![April, May, June],
                        indoor: vec![],
                    },
                },
            ],
            sun_requirement: vec![SunExposure::FullSun],
            soil_types: vec![SoilType::Sandy, SoilType::Chalky, SoilType::Loamy],
            spacing_cm: 30,
            days_to_harvest: 90,
            days_to_plant: 28,
            lifecycle: Lifecycle::Perennial,
            good_companions: vec!["cabbage".into(), "tomato".into(), "eggplant".into()],
            bad_companions: vec!["basil".into()],
            beginner_friendly: true,
            category: Category::Herb,
        },
        Vegetable {
            id: "rosemary".into(),
            name: "Rosemary".into(),
            latin_name: "Salvia rosmarinus".into(),
            calendars: vec![
                RegionCalendar {
                    region: Region::Mediterranean,
                    sowing: CalendarWindow {
                        outdoor: vec![
                            March, April, May, June, July, August, September, October, November,
                        ],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![March, April, May, September, October, November],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Temperate,
                    sowing: CalendarWindow {
                        outdoor: vec![March, April, May, June, July, August, September],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![April, May, June, September],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Oceanic,
                    sowing: CalendarWindow {
                        outdoor: vec![March, April, May, June, July, August, September, October],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![April, May, June, September, October, November],
                        indoor: vec![],
                    },
                },
            ],
            sun_requirement: vec![SunExposure::FullSun],
            soil_types: vec![SoilType::Sandy, SoilType::Chalky],
            spacing_cm: 40,
            days_to_harvest: 90,
            days_to_plant: 28,
            lifecycle: Lifecycle::Perennial,
            good_companions: vec!["cabbage".into(), "green-bean".into(), "sage".into()],
            bad_companions: vec!["cucumber".into(), "pumpkin".into()],
            beginner_friendly: true,
            category: Category::Herb,
        },
        Vegetable {
            id: "beet".into(),
            name: "Beet".into(),
            latin_name: "Beta vulgaris".into(),
            calendars: vec![
                RegionCalendar {
                    region: Region::Temperate,
                    sowing: CalendarWindow {
                        outdoor: vec![March, April, May, June, July],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![April, May, June, July],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Oceanic,
                    sowing: CalendarWindow {
                        outdoor: vec![March, April, May, June, July, August],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![April, May, June, July, August],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Continental,
                    sowing: CalendarWindow {
                        outdoor: vec![April, May, June],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![May, June, July],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Mountain,
                    sowing: CalendarWindow {
                        outdoor: vec![May, June, July],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![June, July],
                        indoor: vec![],
                    },
                },
            ],
            sun_requirement: vec![SunExposure::FullSun, SunExposure::PartialShade],
            soil_types: vec![SoilType::Loamy, SoilType::Sandy, SoilType::Clay],
            spacing_cm: 15,
            days_to_harvest: 60,
            days_to_plant: 0,
            lifecycle: Lifecycle::Annual,
            good_companions: vec!["onion".into(), "lettuce".into(), "radish".into()],
            bad_companions: vec!["green-bean".into(), "mustard".into()],
            beginner_friendly: true,
            category: Category::Root,
        },
        Vegetable {
            id: "spinach".into(),
            name: "Spinach".into(),
            latin_name: "Spinacia oleracea".into(),
            calendars: vec![
                RegionCalendar {
                    region: Region::Temperate,
                    sowing: CalendarWindow {
                        outdoor: vec![March, April, May, August, September, October],
                        indoor: vec![January, February],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Oceanic,
                    sowing: CalendarWindow {
                        outdoor: vec![
                            February, March, April, May, August, September, October, November,
                        ],
                        indoor: vec![January, February, November, December],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Continental,
                    sowing: CalendarWindow {
                        outdoor: vec![March, April, May, August, September],
                        indoor: vec![February],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Mountain,
                    sowing: CalendarWindow {
                        outdoor: vec![April, May, June, July, August],
                        indoor: vec![March],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![],
                        indoor: vec![],
                    },
                },
            ],
            sun_requirement: vec![SunExposure::PartialShade, SunExposure::FullSun],
            soil_types: vec![SoilType::Loamy, SoilType::Humus, SoilType::Clay],
            spacing_cm: 15,
            days_to_harvest: 40,
            days_to_plant: 0,
            lifecycle: Lifecycle::Annual,
            good_companions: vec!["strawberry".into(), "tomato".into(), "radish".into()],
            bad_companions: vec!["beet".into(), "sorrel".into()],
            beginner_friendly: true,
            category: Category::Leafy,
        },
        Vegetable {
            id: "fennel".into(),
            name: "Fennel".into(),
            latin_name: "Foeniculum vulgare".into(),
            calendars: vec![
                RegionCalendar {
                    region: Region::Mediterranean,
                    sowing: CalendarWindow {
                        outdoor: vec![March, April, May, June, August, September],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![May, June, September, October],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Temperate,
                    sowing: CalendarWindow {
                        outdoor: vec![April, May, June, July, August],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![May, June, September],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Oceanic,
                    sowing: CalendarWindow {
                        outdoor: vec![April, May, June, July, August, September],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![May, June, September, October],
                        indoor: vec![],
                    },
                },
            ],
            sun_requirement: vec![SunExposure::FullSun],
            soil_types: vec![SoilType::Loamy, SoilType::Sandy],
            spacing_cm: 30,
            days_to_harvest: 90,
            days_to_plant: 21,
            lifecycle: Lifecycle::Perennial,
            good_companions: vec![],
            bad_companions: vec![
                "tomato".into(),
                "green-bean".into(),
                "pepper".into(),
                "carrot".into(),
                "cabbage".into(),
                "pea".into(),
                "cucumber".into(),
            ],
            beginner_friendly: false,
            category: Category::Herb,
        },
        Vegetable {
            id: "eggplant".into(),
            name: "Eggplant".into(),
            latin_name: "Solanum melongena".into(),
            calendars: vec![
                RegionCalendar {
                    region: Region::Mediterranean,
                    sowing: CalendarWindow {
                        outdoor: vec![March, April],
                        indoor: vec![February, March],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![April, May, June],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Temperate,
                    sowing: CalendarWindow {
                        outdoor: vec![April, May],
                        indoor: vec![February, March],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![May, June],
                        indoor: vec![],
                    },
                },
            ],
            sun_requirement: vec![SunExposure::FullSun],
            soil_types: vec![SoilType::Loamy, SoilType::Humus],
            spacing_cm: 60,
            days_to_harvest: 75,
            days_to_plant: 60,
            lifecycle: Lifecycle::Annual,
            good_companions: vec!["basil".into(), "thyme".into(), "pepper".into()],
            bad_companions: vec!["fennel".into()],
            beginner_friendly: false,
            category: Category::Fruit,
        },
        Vegetable {
            id: "celery".into(),
            name: "Celery".into(),
            latin_name: "Apium graveolens".into(),
            calendars: vec![
                RegionCalendar {
                    region: Region::Temperate,
                    sowing: CalendarWindow {
                        outdoor: vec![March, April, May, June],
                        indoor: vec![February, March],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![May, June],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Oceanic,
                    sowing: CalendarWindow {
                        outdoor: vec![March, April, May, June],
                        indoor: vec![February, March],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![April, May, June],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Continental,
                    sowing: CalendarWindow {
                        outdoor: vec![April, May, June],
                        indoor: vec![March],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![May, June],
                        indoor: vec![],
                    },
                },
            ],
            sun_requirement: vec![SunExposure::FullSun, SunExposure::PartialShade],
            soil_types: vec![SoilType::Loamy, SoilType::Humus, SoilType::Clay],
            spacing_cm: 30,
            days_to_harvest: 100,
            days_to_plant: 60,
            lifecycle: Lifecycle::Biennial,
            good_companions: vec!["leek".into(), "cabbage".into(), "tomato".into()],
            bad_companions: vec!["lettuce".into(), "garlic".into()],
            beginner_friendly: false,
            category: Category::Leafy,
        },
        Vegetable {
            id: "potato".into(),
            name: "Potato".into(),
            latin_name: "Solanum tuberosum".into(),
            calendars: vec![
                RegionCalendar {
                    region: Region::Temperate,
                    sowing: CalendarWindow {
                        outdoor: vec![],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![March, April, May],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Oceanic,
                    sowing: CalendarWindow {
                        outdoor: vec![],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![March, April, May],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Continental,
                    sowing: CalendarWindow {
                        outdoor: vec![],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![April, May, June],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Mountain,
                    sowing: CalendarWindow {
                        outdoor: vec![],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![May, June],
                        indoor: vec![],
                    },
                },
            ],
            sun_requirement: vec![SunExposure::FullSun],
            soil_types: vec![SoilType::Loamy, SoilType::Sandy, SoilType::Humus],
            spacing_cm: 35,
            days_to_harvest: 90,
            days_to_plant: 0,
            lifecycle: Lifecycle::Annual,
            good_companions: vec!["green-bean".into(), "cabbage".into(), "maïs".into()],
            bad_companions: vec!["tomato".into(), "cucumber".into(), "zucchini".into()],
            beginner_friendly: true,
            category: Category::Root,
        },
        Vegetable {
            id: "maïs".into(),
            name: "Corn".into(),
            latin_name: "Zea mays".into(),
            calendars: vec![
                RegionCalendar {
                    region: Region::Temperate,
                    sowing: CalendarWindow {
                        outdoor: vec![April, May],
                        indoor: vec![March, April],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![May, June],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Mediterranean,
                    sowing: CalendarWindow {
                        outdoor: vec![March, April],
                        indoor: vec![February, March],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![April, May],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Continental,
                    sowing: CalendarWindow {
                        outdoor: vec![April, May],
                        indoor: vec![March, April],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![May, June],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Oceanic,
                    sowing: CalendarWindow {
                        outdoor: vec![April, May],
                        indoor: vec![March, April],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![May, June],
                        indoor: vec![],
                    },
                },
            ],
            sun_requirement: vec![SunExposure::FullSun],
            soil_types: vec![SoilType::Loamy, SoilType::Humus, SoilType::Clay],
            spacing_cm: 40,
            days_to_harvest: 80,
            days_to_plant: 0,
            lifecycle: Lifecycle::Annual,
            good_companions: vec!["green-bean".into(), "zucchini".into(), "potato".into()],
            bad_companions: vec!["tomato".into(), "celery".into()],
            beginner_friendly: true,
            category: Category::Pod,
        },
        Vegetable {
            id: "pumpkin".into(),
            name: "Pumpkin".into(),
            latin_name: "Cucurbita maxima".into(),
            calendars: vec![
                RegionCalendar {
                    region: Region::Temperate,
                    sowing: CalendarWindow {
                        outdoor: vec![April, May],
                        indoor: vec![March, April],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![May, June],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Mediterranean,
                    sowing: CalendarWindow {
                        outdoor: vec![March, April],
                        indoor: vec![February, March],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![April, May],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Oceanic,
                    sowing: CalendarWindow {
                        outdoor: vec![April, May],
                        indoor: vec![March, April],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![May, June],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Continental,
                    sowing: CalendarWindow {
                        outdoor: vec![April, May],
                        indoor: vec![March, April],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![May, June],
                        indoor: vec![],
                    },
                },
            ],
            sun_requirement: vec![SunExposure::FullSun],
            soil_types: vec![SoilType::Loamy, SoilType::Humus],
            spacing_cm: 120,
            days_to_harvest: 100,
            days_to_plant: 21,
            lifecycle: Lifecycle::Annual,
            good_companions: vec!["maïs".into(), "green-bean".into(), "onion".into()],
            bad_companions: vec!["potato".into(), "rosemary".into()],
            beginner_friendly: false,
            category: Category::Fruit,
        },
        Vegetable {
            id: "chive".into(),
            name: "Chive".into(),
            latin_name: "Allium schoenoprasum".into(),
            calendars: vec![
                RegionCalendar {
                    region: Region::Temperate,
                    sowing: CalendarWindow {
                        outdoor: vec![March, April, May, June, July, August],
                        indoor: vec![January, February],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![April, May, June],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Oceanic,
                    sowing: CalendarWindow {
                        outdoor: vec![February, March, April, May, June, July, August],
                        indoor: vec![January, February],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![March, April, May, June],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Continental,
                    sowing: CalendarWindow {
                        outdoor: vec![April, May, June, July],
                        indoor: vec![February, March],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![April, May, June],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Mountain,
                    sowing: CalendarWindow {
                        outdoor: vec![April, May, June, July],
                        indoor: vec![March, April],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![May, June],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Mediterranean,
                    sowing: CalendarWindow {
                        outdoor: vec![March, April, May, June, July, August, September],
                        indoor: vec![January, February],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![March, April, May, June, July],
                        indoor: vec![],
                    },
                },
            ],
            sun_requirement: vec![SunExposure::FullSun, SunExposure::PartialShade],
            soil_types: vec![SoilType::Loamy, SoilType::Humus],
            spacing_cm: 20,
            days_to_harvest: 60,
            days_to_plant: 21,
            lifecycle: Lifecycle::Perennial,
            good_companions: vec![
                "carrot".into(),
                "tomato".into(),
                "rose".into(),
                "strawberry".into(),
            ],
            bad_companions: vec!["green-bean".into(), "pea".into()],
            beginner_friendly: true,
            category: Category::Herb,
        },
        Vegetable {
            id: "mint".into(),
            name: "Mint".into(),
            latin_name: "Mentha".into(),
            calendars: vec![
                RegionCalendar {
                    region: Region::Temperate,
                    sowing: CalendarWindow {
                        outdoor: vec![March, April, May, June, July],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![April, May, June],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Oceanic,
                    sowing: CalendarWindow {
                        outdoor: vec![March, April, May, June, July, August],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![April, May, June, July],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Continental,
                    sowing: CalendarWindow {
                        outdoor: vec![April, May, June, July],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![April, May, June],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Mediterranean,
                    sowing: CalendarWindow {
                        outdoor: vec![March, April, May, June, July, August, September],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![March, April, May, June, July, August],
                        indoor: vec![],
                    },
                },
            ],
            sun_requirement: vec![SunExposure::PartialShade, SunExposure::FullSun],
            soil_types: vec![SoilType::Loamy, SoilType::Humus, SoilType::Clay],
            spacing_cm: 30,
            days_to_harvest: 60,
            days_to_plant: 0,
            lifecycle: Lifecycle::Perennial,
            good_companions: vec!["cabbage".into(), "tomato".into(), "pea".into()],
            bad_companions: vec!["parsley".into()],
            beginner_friendly: true,
            category: Category::Herb,
        },
        Vegetable {
            id: "strawberry".into(),
            name: "Strawberry".into(),
            latin_name: "Fragaria × ananassa".into(),
            calendars: vec![
                RegionCalendar {
                    region: Region::Temperate,
                    sowing: CalendarWindow {
                        outdoor: vec![],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![March, April, May, August, September],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Oceanic,
                    sowing: CalendarWindow {
                        outdoor: vec![],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![March, April, May, August, September, October],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Continental,
                    sowing: CalendarWindow {
                        outdoor: vec![],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![April, May, August, September],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Mountain,
                    sowing: CalendarWindow {
                        outdoor: vec![],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![May, June],
                        indoor: vec![],
                    },
                },
            ],
            sun_requirement: vec![SunExposure::FullSun, SunExposure::PartialShade],
            soil_types: vec![SoilType::Loamy, SoilType::Sandy, SoilType::Humus],
            spacing_cm: 30,
            days_to_harvest: 90,
            days_to_plant: 30,
            lifecycle: Lifecycle::Perennial,
            good_companions: vec![
                "lettuce".into(),
                "spinach".into(),
                "garlic".into(),
                "onion".into(),
            ],
            bad_companions: vec!["cabbage".into(), "fennel".into()],
            beginner_friendly: true,
            category: Category::Fruit,
        },
        Vegetable {
            id: "turnip".into(),
            name: "Turnip".into(),
            latin_name: "Brassica rapa".into(),
            calendars: vec![
                RegionCalendar {
                    region: Region::Temperate,
                    sowing: CalendarWindow {
                        outdoor: vec![March, April, May, July, August, September],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Oceanic,
                    sowing: CalendarWindow {
                        outdoor: vec![
                            February, March, April, May, July, August, September, October,
                        ],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Continental,
                    sowing: CalendarWindow {
                        outdoor: vec![April, May, August, September],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Mountain,
                    sowing: CalendarWindow {
                        outdoor: vec![May, June, July],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![],
                        indoor: vec![],
                    },
                },
            ],
            sun_requirement: vec![SunExposure::FullSun, SunExposure::PartialShade],
            soil_types: vec![SoilType::Loamy, SoilType::Clay, SoilType::Chalky],
            spacing_cm: 20,
            days_to_harvest: 40,
            days_to_plant: 0,
            lifecycle: Lifecycle::Annual,
            good_companions: vec!["pea".into(), "green-bean".into()],
            bad_companions: vec!["mustard".into(), "radish".into()],
            beginner_friendly: true,
            category: Category::Root,
        },
        Vegetable {
            id: "cauliflower".into(),
            name: "Cauliflower".into(),
            latin_name: "Brassica oleracea var. botrytis".into(),
            calendars: vec![
                RegionCalendar {
                    region: Region::Temperate,
                    sowing: CalendarWindow {
                        outdoor: vec![March, April, May, July, August],
                        indoor: vec![January, February],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![May, June, September, October],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Oceanic,
                    sowing: CalendarWindow {
                        outdoor: vec![March, April, July, August],
                        indoor: vec![January, February],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![April, May, September, October, November],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Continental,
                    sowing: CalendarWindow {
                        outdoor: vec![April, May, July, August],
                        indoor: vec![February, March],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![May, June, September],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Mountain,
                    sowing: CalendarWindow {
                        outdoor: vec![May, June],
                        indoor: vec![March, April],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![June, July],
                        indoor: vec![],
                    },
                },
            ],
            sun_requirement: vec![SunExposure::FullSun],
            soil_types: vec![SoilType::Loamy, SoilType::Clay, SoilType::Chalky],
            spacing_cm: 60,
            days_to_harvest: 80,
            days_to_plant: 35,
            lifecycle: Lifecycle::Annual,
            good_companions: vec!["celery".into(), "onion".into(), "chive".into()],
            bad_companions: vec!["tomato".into(), "strawberry".into()],
            beginner_friendly: false,
            category: Category::Leafy,
        },
        Vegetable {
            id: "red-pepper".into(),
            name: "Red Pepper".into(),
            latin_name: "Capsicum annuum".into(),
            calendars: vec![
                RegionCalendar {
                    region: Region::Mediterranean,
                    sowing: CalendarWindow {
                        outdoor: vec![March, April],
                        indoor: vec![January, February, March],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![April, May],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Temperate,
                    sowing: CalendarWindow {
                        outdoor: vec![April, May],
                        indoor: vec![February, March],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![May, June],
                        indoor: vec![],
                    },
                },
            ],
            sun_requirement: vec![SunExposure::FullSun],
            soil_types: vec![SoilType::Loamy, SoilType::Humus],
            spacing_cm: 50,
            days_to_harvest: 70,
            days_to_plant: 60,
            lifecycle: Lifecycle::Annual,
            good_companions: vec!["basil".into(), "carrot".into(), "eggplant".into()],
            bad_companions: vec!["fennel".into()],
            beginner_friendly: false,
            category: Category::Fruit,
        },
        Vegetable {
            id: "asparagus".into(),
            name: "Asparagus".into(),
            latin_name: "Asparagus officinalis".into(),
            calendars: vec![
                RegionCalendar {
                    region: Region::Temperate,
                    sowing: CalendarWindow {
                        outdoor: vec![March, April],
                        indoor: vec![February],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![April, May],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Oceanic,
                    sowing: CalendarWindow {
                        outdoor: vec![March, April],
                        indoor: vec![February, March],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![April, May],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Continental,
                    sowing: CalendarWindow {
                        outdoor: vec![April, May],
                        indoor: vec![March],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![May],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Mediterranean,
                    sowing: CalendarWindow {
                        outdoor: vec![February, March],
                        indoor: vec![January, February],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![March, April, May],
                        indoor: vec![],
                    },
                },
            ],
            sun_requirement: vec![SunExposure::FullSun],
            soil_types: vec![SoilType::Sandy, SoilType::Loamy],
            spacing_cm: 45,
            days_to_harvest: 730,
            days_to_plant: 90,
            lifecycle: Lifecycle::Perennial,
            good_companions: vec!["tomato".into(), "parsley".into(), "basil".into()],
            bad_companions: vec!["onion".into(), "garlic".into()],
            beginner_friendly: false,
            category: Category::Leafy,
        },
        Vegetable {
            id: "artichoke".into(),
            name: "Artichoke".into(),
            latin_name: "Cynara cardunculus var. scolymus".into(),
            calendars: vec![
                RegionCalendar {
                    region: Region::Mediterranean,
                    sowing: CalendarWindow {
                        outdoor: vec![March, April, May],
                        indoor: vec![February, March],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![April, May, June],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Oceanic,
                    sowing: CalendarWindow {
                        outdoor: vec![April, May],
                        indoor: vec![March, April],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![May, June, July],
                        indoor: vec![],
                    },
                },
            ],
            sun_requirement: vec![SunExposure::FullSun],
            soil_types: vec![SoilType::Loamy, SoilType::Humus],
            spacing_cm: 80,
            days_to_harvest: 365,
            days_to_plant: 60,
            lifecycle: Lifecycle::Perennial,
            good_companions: vec!["cabbage".into(), "lettuce".into()],
            bad_companions: vec!["green-bean".into(), "tomato".into()],
            beginner_friendly: false,
            category: Category::Leafy,
        },
    ]
}
