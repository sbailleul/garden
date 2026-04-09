import { Link } from "@tanstack/react-router";

import type { CompanionInfo } from "@/api/types";
import { Badge } from "@/components/ui/badge";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";

type Props = {
  vegetableId: string;
  vegetableName: string;
  good: CompanionInfo[];
  bad: CompanionInfo[];
};

export function CompanionList({ vegetableId, vegetableName, good, bad }: Props) {
  return (
    <div className="space-y-4">
      <div className="flex items-center gap-3">
        <Link
          to="/vegetables/$id"
          params={{ id: vegetableId }}
          className="text-muted-foreground text-sm hover:underline"
        >
          ← {vegetableName}
        </Link>
      </div>

      <h1 className="text-2xl font-bold">Companions for {vegetableName}</h1>

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
