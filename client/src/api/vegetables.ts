import { apiClient } from "@/api/client";
import type {
  CompanionsApiResponse,
  VegetableApiResponse,
  VegetablesApiResponse,
} from "@/api/types";

export interface VegetableListParams {
  page?: number;
  size?: number;
}

export async function fetchVegetables(
  params?: VegetableListParams,
): Promise<VegetablesApiResponse> {
  const { data, error } = await apiClient.GET("/api/vegetables", {
    params: { query: params },
  });
  if (error) throw new Error(JSON.stringify(error));
  return data as unknown as VegetablesApiResponse;
}

export async function fetchVegetable(id: string): Promise<VegetableApiResponse> {
  const { data, error } = await apiClient.GET("/api/vegetables/{id}", {
    params: { path: { id } },
  });
  if (error) throw new Error(JSON.stringify(error));
  return data as unknown as VegetableApiResponse;
}

export async function fetchCompanions(id: string): Promise<CompanionsApiResponse> {
  const { data, error } = await apiClient.GET("/api/vegetables/{id}/companions", {
    params: { path: { id } },
  });
  if (error) throw new Error(JSON.stringify(error));
  return data as unknown as CompanionsApiResponse;
}
