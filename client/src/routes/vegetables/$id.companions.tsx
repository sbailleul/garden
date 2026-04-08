import { createFileRoute, Link } from "@tanstack/react-router";
import { useSuspenseQuery } from "@tanstack/react-query";

import { fetchCompanions } from "../../api/vegetables";
import { Badge } from "../../components/ui/badge";
import { Card, CardContent, CardHeader, CardTitle } from "../../components/ui/card";

export const Route = createFileRoute("/vegetables/$id/companions")({
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
    <div className="space-y-4">
      <div className="flex items-center gap-3">
        <Link
          to="/vegetables/$id"
          params={{ id }}
          className="text-muted-foreground text-sm hover:underline"
        >
          ← {name}
        </Link>
      </div>

      <h1 className="text-2xl font-bold">Companions for {name}</h1>

      <div className="grid gap-4 sm:grid-cols-2">
        <Card>
          <CardHeader>
            <CardTitle className="text-base text-green-700">Good companions</CardTitle>
          </CardHeader>
          <CardContent>
            {good.length === 0 ? (
              <p className="text-muted-foreground text-sm">None listed.</p>
            ) : (
              <div className="flex flex-wrap gap-2">
                {good.map((c) => (
                  <Link key={c.id} to="/vegetables/$id" params={{ id: c.id }}>
                    <Badge variant="success">{c.name}</Badge>
                  </Link>
                ))}
              </div>
            )}
          </CardContent>
        </Card>

        <Card>
          <CardHeader>
            <CardTitle className="text-base text-red-700">Bad companions</CardTitle>
          </CardHeader>
          <CardContent>
            {bad.length === 0 ? (
              <p className="text-muted-foreground text-sm">None listed.</p>
            ) : (
              <div className="flex flex-wrap gap-2">
                {bad.map((c) => (
                  <Link key={c.id} to="/vegetables/$id" params={{ id: c.id }}>
                    <Badge variant="destructive">{c.name}</Badge>
                  </Link>
                ))}
              </div>
            )}
          </CardContent>
        </Card>
      </div>
    </div>
  );
}
