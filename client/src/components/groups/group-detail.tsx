import { Link } from "@tanstack/react-router";

import type { Group, Vegetable } from "@/api/types";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";

type Props = {
  group: Group;
  vegetables: Vegetable[];
};

export function GroupDetail({ group, vegetables }: Props) {
  return (
    <div className="space-y-6">
      <div>
        <Link
          to="/groups"
          className="text-muted-foreground text-sm hover:underline"
        >
          ← Groups
        </Link>
      </div>

      <div>
        <h1 className="text-2xl font-bold">{group.name}</h1>
        <p className="text-muted-foreground text-sm font-mono">{group.id}</p>
      </div>

      <div>
        <h2 className="mb-2 text-lg font-semibold">Vegetables</h2>
        {vegetables.length === 0 ? (
          <p className="text-muted-foreground text-sm">No vegetables in this group.</p>
        ) : (
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead>Name</TableHead>
                <TableHead>ID</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              {vegetables.map((v) => (
                <TableRow key={v.id}>
                  <TableCell>
                    <Link
                      to="/varieties"
                      className="font-medium hover:underline"
                    >
                      {v.name}
                    </Link>
                  </TableCell>
                  <TableCell className="text-muted-foreground text-sm font-mono">
                    {v.id}
                  </TableCell>
                </TableRow>
              ))}
            </TableBody>
          </Table>
        )}
      </div>
    </div>
  );
}
