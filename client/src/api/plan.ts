import type { components } from "./schema.d.ts";
import { apiClient } from "./client";

export type PlanRequest = components["schemas"]["PlanRequest"];
export type PlanApiResponse = components["schemas"]["PlanApiResponse"];
export type PlanResponse = components["schemas"]["PlanResponse"];
export type WeeklyPlan = components["schemas"]["WeeklyPlan"];
export type PlannedCell = components["schemas"]["PlannedCell"];
export type Period = components["schemas"]["Period"];
export type LayoutCell = components["schemas"]["LayoutCell"];

export async function postPlan(body: PlanRequest): Promise<PlanApiResponse> {
  const { data, error } = await apiClient.POST("/api/plan", { body });
  if (error) throw new Error(JSON.stringify(error));
  return data;
}
