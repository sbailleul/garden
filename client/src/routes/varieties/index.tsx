import { createFileRoute } from "@tanstack/react-router";
import { useSuspenseQuery } from "@tanstack/react-query";

import { fetchVarieties } from "@/api/varieties";
import { VarietyTable } from "@/components/varieties/variety-table";

export const Route = createFileRoute("/varieties/")({
  loader: ({ context: { queryClient } }) =>
    queryClient.ensureQueryData({
      queryKey: ["varieties"],
      queryFn: fetchVarieties,
    }),
  component: VarietyCatalogue,
});

function VarietyCatalogue() {
  const { data } = useSuspenseQuery({
    queryKey: ["varieties"],
    queryFn: fetchVarieties,
  });

  return <VarietyTable varieties={data.payload.map((v) => v.payload)} />;
}
