import { createFileRoute } from "@tanstack/react-router";
import { useSuspenseQuery } from "@tanstack/react-query";

import { fetchVegetable } from "@/api/vegetables";
import { VegetableDetail } from "@/components/vegetables/vegetable-detail";

export const Route = createFileRoute("/vegetables/$id")({
  loader: ({ context: { queryClient }, params }) =>
    queryClient.ensureQueryData({
      queryKey: ["vegetables", params.id],
      queryFn: () => fetchVegetable(params.id),
    }),
  component: VegetableDetailPage,
});

function VegetableDetailPage() {
  const { id } = Route.useParams();
  const { data } = useSuspenseQuery({
    queryKey: ["vegetables", id],
    queryFn: () => fetchVegetable(id),
  });

  return <VegetableDetail vegetable={data.payload} />;
}
