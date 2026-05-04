import { createFileRoute } from "@tanstack/react-router";
import { useSuspenseQuery } from "@tanstack/react-query";

import { fetchGroups } from "@/api/groups";
import { GroupTable } from "@/components/groups/group-table";

export const Route = createFileRoute("/groups/")({
  loader: ({ context: { queryClient } }) =>
    queryClient.ensureQueryData({
      queryKey: ["groups"],
      queryFn: fetchGroups,
    }),
  component: GroupCatalogue,
});

function GroupCatalogue() {
  const { data } = useSuspenseQuery({
    queryKey: ["groups"],
    queryFn: fetchGroups,
  });

  return <GroupTable groups={data.payload.map((g) => g.payload)} />;
}
