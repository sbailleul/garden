import { createFileRoute } from "@tanstack/react-router";
import { useSuspenseQuery } from "@tanstack/react-query";

import { fetchGroup, fetchVegetablesByGroup } from "@/api/groups";
import { GroupDetail } from "@/components/groups/group-detail";

export const Route = createFileRoute("/groups/$id")({
  loader: ({ context: { queryClient }, params }) =>
    Promise.all([
      queryClient.ensureQueryData({
        queryKey: ["groups", params.id],
        queryFn: () => fetchGroup(params.id),
      }),
      queryClient.ensureQueryData({
        queryKey: ["groups", params.id, "vegetables"],
        queryFn: () => fetchVegetablesByGroup(params.id),
      }),
    ]),
  component: GroupDetailPage,
});

function GroupDetailPage() {
  const { id } = Route.useParams();
  const { data: groupData } = useSuspenseQuery({
    queryKey: ["groups", id],
    queryFn: () => fetchGroup(id),
  });
  const { data: vegetablesData } = useSuspenseQuery({
    queryKey: ["groups", id, "vegetables"],
    queryFn: () => fetchVegetablesByGroup(id),
  });

  return (
    <GroupDetail
      group={groupData.payload}
      vegetables={vegetablesData.payload.map((v) => v.payload)}
    />
  );
}
