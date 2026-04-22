import { createFileRoute } from "@tanstack/react-router";
import { useSuspenseQuery } from "@tanstack/react-query";

import { fetchVariety } from "@/api/varieties";
import { VarietyDetail } from "@/components/varieties/variety-detail";

export const Route = createFileRoute("/varieties/$id")({
  loader: ({ context: { queryClient }, params }) =>
    queryClient.ensureQueryData({
      queryKey: ["varieties", params.id],
      queryFn: () => fetchVariety(params.id),
    }),
  component: VarietyDetailPage,
});

function VarietyDetailPage() {
  const { id } = Route.useParams();
  const { data } = useSuspenseQuery({
    queryKey: ["varieties", id],
    queryFn: () => fetchVariety(id),
  });

  return <VarietyDetail variety={data.payload} />;
}
