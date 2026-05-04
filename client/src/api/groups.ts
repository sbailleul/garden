import { apiClient } from "@/api/client";
import type {
  GroupsApiResponse,
  GroupApiResponse,
  VegetablesApiResponse,
} from "@/api/types";

export async function fetchGroups(): Promise<GroupsApiResponse> {
  const { data, error } = await apiClient.GET("/api/groups");
  if (error) throw new Error(JSON.stringify(error));
  return data as unknown as GroupsApiResponse;
}

export async function fetchGroup(id: string): Promise<GroupApiResponse> {
  const { data, error } = await apiClient.GET("/api/groups/{id}", {
    params: { path: { id } },
  });
  if (error) throw new Error(JSON.stringify(error));
  return data as unknown as GroupApiResponse;
}

export async function fetchVegetablesByGroup(
  id: string,
): Promise<VegetablesApiResponse> {
  const { data, error } = await apiClient.GET("/api/groups/{id}/vegetables", {
    params: { path: { id } },
  });
  if (error) throw new Error(JSON.stringify(error));
  return data as unknown as VegetablesApiResponse;
}
