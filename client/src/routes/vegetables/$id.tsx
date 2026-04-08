import { createFileRoute, Link } from "@tanstack/react-router";
import { useSuspenseQuery } from "@tanstack/react-query";

import { fetchVegetable } from "@/api/vegetables";
import { Badge } from "@/components/ui/badge";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";

export const Route = createFileRoute("/vegetables/$id")({
  loader: ({ context: { queryClient }, params }) =>
    queryClient.ensureQueryData({
      queryKey: ["vegetables", params.id],
      queryFn: () => fetchVegetable(params.id),
    }),
  component: VegetableDetail,
});

function VegetableDetail() {
  const { id } = Route.useParams();
  const { data } = useSuspenseQuery({
    queryKey: ["vegetables", id],
    queryFn: () => fetchVegetable(id),
  });

  const veg = data.payload;

  return (
    <div className="space-y-4">
      <div className="flex items-center gap-3">
        <Link to="/vegetables" className="text-muted-foreground text-sm hover:underline">
          ← Vegetables
        </Link>
      </div>

      <div className="flex items-start justify-between">
        <div>
          <h1 className="text-2xl font-bold">{veg.name}</h1>
          <p className="text-muted-foreground italic">{veg.latinName}</p>
        </div>
        <Link
          to="/vegetables/$id/companions"
          params={{ id }}
          className="inline-flex items-center rounded-md border px-3 py-1.5 text-sm font-medium shadow-sm hover:bg-accent"
        >
          View companions
        </Link>
      </div>

      <div className="grid gap-4 sm:grid-cols-2">
        <Card>
          <CardHeader>
            <CardTitle className="text-base">Details</CardTitle>
          </CardHeader>
          <CardContent className="space-y-2 text-sm">
            <Row label="Category" value={<Badge variant="secondary">{veg.category}</Badge>} />
            <Row label="Lifecycle" value={veg.lifecycle} />
            <Row label="Spacing" value={`${veg.spacingCm} cm`} />
            <Row label="Days to harvest" value={veg.daysToHarvest} />
            <Row label="Days to plant" value={veg.daysToPlant} />
            <Row label="Beginner-friendly" value={veg.beginnerFriendly ? "Yes" : "No"} />
          </CardContent>
        </Card>

        <Card>
          <CardHeader>
            <CardTitle className="text-base">Growing conditions</CardTitle>
          </CardHeader>
          <CardContent className="space-y-2 text-sm">
            <Row
              label="Sun"
              value={
                <div className="flex flex-wrap gap-1">
                  {veg.sunRequirement.map((s) => (
                    <Badge key={s} variant="outline">
                      {s}
                    </Badge>
                  ))}
                </div>
              }
            />
            <Row
              label="Soil"
              value={
                <div className="flex flex-wrap gap-1">
                  {veg.soilTypes.map((s) => (
                    <Badge key={s} variant="outline">
                      {s}
                    </Badge>
                  ))}
                </div>
              }
            />
          </CardContent>
        </Card>
      </div>

      <Card>
        <CardHeader>
          <CardTitle className="text-base">Calendars</CardTitle>
        </CardHeader>
        <CardContent className="space-y-3 text-sm">
          {veg.calendars.map((cal) => (
            <div key={cal.region}>
              <p className="font-medium">{cal.region}</p>
              <p>Sowing indoors: {cal.sowing.indoor.join(", ") || "—"}</p>
              <p>Sowing outdoors: {cal.sowing.outdoor.join(", ") || "—"}</p>
              <p>Planting: {cal.planting.outdoor.join(", ") || "—"}</p>
            </div>
          ))}
        </CardContent>
      </Card>
    </div>
  );
}

function Row({ label, value }: { label: string; value: React.ReactNode }) {
  return (
    <div className="flex items-start gap-2">
      <dt className="text-muted-foreground w-32 shrink-0">{label}</dt>
      <dd>{value}</dd>
    </div>
  );
}
