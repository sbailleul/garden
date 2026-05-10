import { apiClient } from "@/api/client";
import { paths } from "@/api/schema";
import type { VarietiesApiResponse, VarietyApiResponse } from "@/api/types";

export async function fetchVarieties(
  filter?: paths["/api/varieties"]["get"]["parameters"]["query"],
): Promise<VarietiesApiResponse> {
  const { data, error } = await apiClient.GET("/api/varieties", {
    params: { query: filter },
  });
  if (error) throw new Error(JSON.stringify(error));
  return data;
}

export async function fetchVarietiesByVegetable(
  vegetableId: string,
  filter?: paths["/api/vegetables/{id}/varieties"]["get"]["parameters"]["query"],
): Promise<VarietiesApiResponse> {
  const { data, error } = await apiClient.GET("/api/vegetables/{id}/varieties", {
    params: { path: { id: vegetableId }, query: filter },
  });
  if (error) throw new Error(JSON.stringify(error));
  return data;
}

export async function fetchVariety(id: string): Promise<VarietyApiResponse> {
  const { data, error } = await apiClient.GET("/api/varieties/{id}", {
    params: { path: { id } },
  });
  if (error) throw new Error(JSON.stringify(error));
  return data as unknown as VarietyApiResponse;
}
