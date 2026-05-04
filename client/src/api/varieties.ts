import { apiClient } from "@/api/client";
import type {
  Category,
  Lifecycle,
  Region,
  SoilType,
  SunExposure,
  VarietiesApiResponse,
  VarietyApiResponse,
  CompanionsApiResponse,
} from "@/api/types";

export interface VarietyFilterParams {
  page?: number;
  size?: number;
  category?: Category;
  lifecycle?: Lifecycle;
  beginner_friendly?: boolean;
  sun_requirement?: SunExposure;
  soil_type?: SoilType;
  region?: Region;
  vegetable_id?: string;
}

export async function fetchVarieties(
  filter?: VarietyFilterParams,
): Promise<VarietiesApiResponse> {
  const { data, error } = await apiClient.GET("/api/varieties", {
    params: { query: filter },
  });
  if (error) throw new Error(JSON.stringify(error));
  return data as unknown as VarietiesApiResponse;
}

export async function fetchVarietiesByVegetable(
  vegetableId: string,
  filter?: Omit<VarietyFilterParams, "vegetable_id">,
): Promise<VarietiesApiResponse> {
  const { data, error } = await apiClient.GET("/api/vegetables/{id}/varieties", {
    params: { path: { id: vegetableId }, query: filter },
  });
  if (error) throw new Error(JSON.stringify(error));
  return data as unknown as VarietiesApiResponse;
}

export async function fetchVariety(id: string): Promise<VarietyApiResponse> {
  const { data, error } = await apiClient.GET("/api/varieties/{id}", {
    params: { path: { id } },
  });
  if (error) throw new Error(JSON.stringify(error));
  return data as unknown as VarietyApiResponse;
}

export async function fetchCompanions(id: string): Promise<CompanionsApiResponse> {
  const { data, error } = await apiClient.GET("/api/varieties/{id}/companions", {
    params: { path: { id } },
  });
  if (error) throw new Error(JSON.stringify(error));
  return data;
}
