import { createFileRoute } from "@tanstack/react-router";
import { useSuspenseQuery } from "@tanstack/react-query";

import { fetchVegetables } from "@/api/vegetables";
import { VegetableTable } from "@/components/vegetables/vegetable-table";

export const Route = createFileRoute("/vegetables/")({
  loader: ({ context: { queryClient } }) =>
    queryClient.ensureQueryData({
      queryKey: ["vegetables"],
      queryFn: fetchVegetables,
    }),
  component: VegetableCatalogue,
});

function VegetableCatalogue() {
  const { data } = useSuspenseQuery({
    queryKey: ["vegetables"],
    queryFn: fetchVegetables,
  });

  return <VegetableTable vegetables={data.payload.map((v) => v.payload)} />;
}
