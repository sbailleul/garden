import { apiClient } from "@/api/client";
import type { VegetablesApiResponse, VegetableApiResponse } from "@/api/types";

export async function fetchVegetables(): Promise<VegetablesApiResponse> {
  const { data, error } = await apiClient.GET("/api/vegetables");
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
