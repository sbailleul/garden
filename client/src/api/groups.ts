import { apiClient } from "@/api/client";
import { paths } from "@/api/schema";
import type { GroupsApiResponse, GroupApiResponse, VegetablesApiResponse } from "@/api/types";

export async function fetchGroups(): Promise<GroupsApiResponse> {
  const { data, error } = await apiClient.GET("/api/groups");
  if (error) throw new Error(JSON.stringify(error));
  return data;
}

export async function fetchGroup(
  id: paths["/api/groups/{id}"]["get"]["parameters"]["path"]["id"],
): Promise<GroupApiResponse> {
  const { data, error } = await apiClient.GET("/api/groups/{id}", {
    params: { path: { id } },
  });
  if (error) throw new Error(JSON.stringify(error));
  return data;
}

export async function fetchVegetablesByGroup(
  id: paths["/api/groups/{id}/vegetables"]["get"]["parameters"]["path"]["id"],
): Promise<VegetablesApiResponse> {
  const { data, error } = await apiClient.GET("/api/groups/{id}/vegetables", {
    params: { path: { id } },
  });
  if (error) throw new Error(JSON.stringify(error));
  return data;
}
