import { apiClient } from "@/api/client";
import type {
  VarietiesApiResponse,
  VarietyApiResponse,
  CompanionsApiResponse,
} from "@/api/types";

export async function fetchVarieties(): Promise<VarietiesApiResponse> {
  const { data, error } = await apiClient.GET("/api/varieties");
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
