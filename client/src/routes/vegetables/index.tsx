import { createFileRoute, Link } from "@tanstack/react-router";
import { useSuspenseQuery } from "@tanstack/react-query";
import {
  createColumnHelper,
  flexRender,
  getCoreRowModel,
  getFilteredRowModel,
  useReactTable,
} from "@tanstack/react-table";
import { useState } from "react";

import { fetchVegetables, type Vegetable } from "@/api/vegetables";
import { Badge } from "@/components/ui/badge";
import { Input } from "@/components/ui/input";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";

export const Route = createFileRoute("/vegetables/")({
  loader: ({ context: { queryClient } }) =>
    queryClient.ensureQueryData({
      queryKey: ["vegetables"],
      queryFn: fetchVegetables,
    }),
  component: VegetableCatalogue,
});

const columnHelper = createColumnHelper<Vegetable>();

const columns = [
  columnHelper.accessor("id", {
    header: "ID",
    cell: (info) => (
      <Link
        to="/vegetables/$id"
        params={{ id: info.getValue() }}
        className="text-primary underline-offset-4 hover:underline"
      >
        {info.getValue()}
      </Link>
    ),
  }),
  columnHelper.accessor("name", {
    header: "Name",
    filterFn: "includesString",
  }),
  columnHelper.accessor("category", {
    header: "Category",
    filterFn: "equalsString",
    cell: (info) => <Badge variant="secondary">{info.getValue()}</Badge>,
  }),
  columnHelper.accessor("spacingCm", {
    header: "Spacing (cm)",
  }),
  columnHelper.accessor("lifecycle", {
    header: "Lifecycle",
  }),
  columnHelper.accessor("beginnerFriendly", {
    header: "Beginner?",
    cell: (info) => (info.getValue() ? "✓" : "—"),
  }),
];

const CATEGORIES = ["Fruit", "Produce", "Herb", "Root", "Bulb", "Leafy", "Pod"] as const;

function VegetableCatalogue() {
  const { data } = useSuspenseQuery({
    queryKey: ["vegetables"],
    queryFn: fetchVegetables,
  });

  const [nameFilter, setNameFilter] = useState("");
  const [categoryFilter, setCategoryFilter] = useState("all");

  const columnFilters = [
    ...(nameFilter ? [{ id: "name", value: nameFilter }] : []),
    ...(categoryFilter !== "all" ? [{ id: "category", value: categoryFilter }] : []),
  ];

  const table = useReactTable({
    data: data.payload,
    columns,
    state: { columnFilters },
    getCoreRowModel: getCoreRowModel(),
    getFilteredRowModel: getFilteredRowModel(),
  });

  return (
    <div className="space-y-4">
      <h1 className="text-2xl font-bold">Vegetables</h1>

      <div className="flex gap-3">
        <Input
          placeholder="Filter by name…"
          value={nameFilter}
          onChange={(e) => setNameFilter(e.target.value)}
          className="max-w-xs"
          aria-label="Filter by name"
        />
        <Select value={categoryFilter} onValueChange={setCategoryFilter}>
          <SelectTrigger className="w-40" aria-label="Filter by category">
            <SelectValue placeholder="Category" />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="all">All categories</SelectItem>
            {CATEGORIES.map((c) => (
              <SelectItem key={c} value={c}>
                {c}
              </SelectItem>
            ))}
          </SelectContent>
        </Select>
      </div>

      <div className="rounded-md border">
        <Table>
          <TableHeader>
            {table.getHeaderGroups().map((hg) => (
              <TableRow key={hg.id}>
                {hg.headers.map((header) => (
                  <TableHead key={header.id}>
                    {header.isPlaceholder
                      ? null
                      : flexRender(header.column.columnDef.header, header.getContext())}
                  </TableHead>
                ))}
              </TableRow>
            ))}
          </TableHeader>
          <TableBody>
            {table.getRowModel().rows.length === 0 ? (
              <TableRow>
                <TableCell colSpan={columns.length} className="text-center">
                  No vegetables found.
                </TableCell>
              </TableRow>
            ) : (
              table.getRowModel().rows.map((row) => (
                <TableRow key={row.id}>
                  {row.getVisibleCells().map((cell) => (
                    <TableCell key={cell.id}>
                      {flexRender(cell.column.columnDef.cell, cell.getContext())}
                    </TableCell>
                  ))}
                </TableRow>
              ))
            )}
          </TableBody>
        </Table>
      </div>

      <p className="text-muted-foreground text-sm">
        {table.getRowModel().rows.length} vegetable
        {table.getRowModel().rows.length !== 1 ? "s" : ""}
      </p>
    </div>
  );
}
