import { apiClient } from "@/api/client";
import { paths } from "@/api/schema";
import type {
  CompanionsApiResponse,
  VegetableApiResponse,
  VegetablesApiResponse,
} from "@/api/types";

export async function fetchVegetables(
  params?: paths["/api/vegetables"]["get"]["parameters"]["query"],
): Promise<VegetablesApiResponse> {
  const { data, error } = await apiClient.GET("/api/vegetables", {
    params: { query: params },
  });
  if (error) throw new Error(JSON.stringify(error));
  return data;
}

export async function fetchVegetable(
  id: paths["/api/vegetables/{id}"]["get"]["parameters"]["path"]["id"],
): Promise<VegetableApiResponse> {
  const { data, error } = await apiClient.GET("/api/vegetables/{id}", {
    params: { path: { id } },
  });
  if (error) throw new Error(JSON.stringify(error));
  return data;
}

export async function fetchCompanions(
  id: paths["/api/vegetables/{id}/companions"]["get"]["parameters"]["path"]["id"],
): Promise<CompanionsApiResponse> {
  const { data, error } = await apiClient.GET("/api/vegetables/{id}/companions", {
    params: { path: { id } },
  });
  if (error) throw new Error(JSON.stringify(error));
  return data;
}
