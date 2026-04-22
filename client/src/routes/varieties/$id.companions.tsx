import { createFileRoute } from "@tanstack/react-router";
import { useSuspenseQuery } from "@tanstack/react-query";

import { fetchCompanions } from "@/api/varieties";
import { CompanionList } from "@/components/varieties/companion-list";

export const Route = createFileRoute("/varieties/$id/companions")({
  loader: ({ context: { queryClient }, params }) =>
    queryClient.ensureQueryData({
      queryKey: ["companions", params.id],
      queryFn: () => fetchCompanions(params.id),
    }),
  component: CompanionsPage,
});

function CompanionsPage() {
  const { id } = Route.useParams();
  const { data } = useSuspenseQuery({
    queryKey: ["companions", id],
    queryFn: () => fetchCompanions(id),
  });

  const { name, good, bad } = data.payload;

  return (
    <CompanionList
      varietyId={id}
      varietyName={name}
      good={good}
      bad={bad}
    />
  );
}
