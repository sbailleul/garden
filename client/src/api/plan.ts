import { apiClient } from "@/api/client";
import type { PlanApiResponse, PlanRequest } from "@/api/types";

export async function postPlan(body: PlanRequest): Promise<PlanApiResponse> {
  const { data, error } = await apiClient.POST("/api/plan", { body });
  if (error) throw new Error(JSON.stringify(error));
  return data;
}
