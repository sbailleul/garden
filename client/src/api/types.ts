import type { components } from "@/api/schema.d.ts";

// Primitive / enum types
export type Category = components["schemas"]["Category"];
export type Lifecycle = components["schemas"]["Lifecycle"];
export type Level = components["schemas"]["Level"];
export type Month = components["schemas"]["Month"];
export type Region = components["schemas"]["Region"];
export type SoilType = components["schemas"]["SoilType"];
export type SunExposure = components["schemas"]["SunExposure"];

// Domain model types
export type CalendarWindow = components["schemas"]["CalendarWindow"];
export type CompanionInfo = components["schemas"]["CompanionInfo"];
export type Coordinate = components["schemas"]["Coordinate"];
export type LayoutCell = components["schemas"]["LayoutCell"];
export type Link = components["schemas"]["Link"];
export type Pagination = components["schemas"]["Pagination"];
export type Period = components["schemas"]["Period"];
export type PlannedCell = components["schemas"]["PlannedCell"];
export type PreferenceEntry = components["schemas"]["PreferenceEntry"];
export type RegionCalendar = components["schemas"]["RegionCalendar"];
export type SowingRecord = components["schemas"]["SowingRecord"];
export type SowingTask = components["schemas"]["SowingTask"];
export type Group = components["schemas"]["Group"];
export type Variety = components["schemas"]["Variety"];
export type Vegetable = components["schemas"]["Vegetable"];

// Request / response envelope types
export type GroupApiResponse = components["schemas"]["GroupApiResponse"];
export type GroupsApiResponse = components["schemas"]["GroupsApiResponse"];
export type ErrorResponse = components["schemas"]["ErrorResponse"];
export type CompanionsResponse = components["schemas"]["CompanionsResponse"];
export type CompanionsApiResponse = components["schemas"]["CompanionsApiResponse"];
export type PlanRequest = components["schemas"]["PlanRequest"];
export type PlanRequestLayout = PlanRequest["layout"];
export type PlanResponse = components["schemas"]["PlanResponse"];
export type PlanApiResponse = components["schemas"]["PlanApiResponse"];
export type VarietyApiResponse = components["schemas"]["VarietyApiResponse"];
export type VarietiesApiResponse = components["schemas"]["VarietiesApiResponse"];
export type VegetableApiResponse = components["schemas"]["VegetableApiResponse"];
export type VegetablesApiResponse = components["schemas"]["VegetablesApiResponse"];
export type WeeklyPlan = components["schemas"]["WeeklyPlan"];
