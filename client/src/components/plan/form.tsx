import { useForm } from "@tanstack/react-form";

import type {
  Level,
  PlanApiResponse,
  PlanRequest,
  PlanRequestLayout,
  PlannedCell,
  Region,
  SoilType,
  SunExposure,
  WeeklyPlan
} from "@/api/types";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";

const REGIONS: Region[] = ["Temperate", "Mediterranean", "Oceanic", "Continental", "Mountain"];
const SUN_OPTIONS: SunExposure[] = ["FullSun", "PartialShade", "Shade"];
const SOIL_OPTIONS: SoilType[] = ["Clay", "Sandy", "Loamy", "Chalky", "Humus"];
const LEVEL_OPTIONS: Level[] = ["Beginner", "Expert"];

const CATEGORY_COLOURS: Record<string, string> = {
  Fruit: "bg-red-100 text-red-800",
  Produce: "bg-green-100 text-green-800",
  Herb: "bg-lime-100 text-lime-800",
  Root: "bg-orange-100 text-orange-800",
  Bulb: "bg-purple-100 text-purple-800",
  Leafy: "bg-emerald-100 text-emerald-800",
  Pod: "bg-yellow-100 text-yellow-800",
};

function cellColour(cell: PlannedCell): string {
  if (cell.type === "Empty") return "bg-muted/30";
  if (cell.type === "Blocked") return "bg-gray-300";
  if (cell.type === "Overflowed") return "bg-blue-50";
  return CATEGORY_COLOURS["Fruit"] ?? "bg-blue-100 text-blue-800";
}

function cellLabel(cell: PlannedCell): string {
  if (cell.type === "SelfContained" || cell.type === "Overflowing") {
    return cell.name.slice(0, 3).toUpperCase();
  }
  if (cell.type === "Blocked") return "✕";
  return "";
}

function WeekGrid({ week }: { week: WeeklyPlan }) {
  return (
    <div className="space-y-2">
      <p className="text-sm font-medium">
        Week {week.weekCount}: {week.period.start} → {week.period.end}
        <span className="text-muted-foreground ml-2">(score: {week.score})</span>
      </p>
      <div
        className="inline-grid gap-0.5"
        style={{ gridTemplateColumns: `repeat(${week.grid[0]?.length ?? 1}, 2rem)` }}
        role="grid"
        aria-label={`Week ${week.weekCount} garden grid`}
      >
        {week.grid.flatMap((row, r) =>
          row.map((cell, c) => (
            <div
              key={`${r}-${c}`}
              className={[
                "flex h-8 w-8 items-center justify-center rounded-sm text-xs font-bold",
                cellColour(cell),
              ].join(" ")}
              title={
                cell.type !== "Empty" && cell.type !== "Blocked"
                  ? cell.type === "Overflowed"
                    ? `Overflowed by (${cell.coveredBy.row},${cell.coveredBy.col})`
                    : `${cell.name} — ${cell.reason}`
                  : cell.type
              }
            >
              {cellLabel(cell)}
            </div>
          )),
        )}
      </div>
    </div>
  );
}

type Props = {
  onSubmit: (body: PlanRequest) => void;
  isPending: boolean;
  isError: boolean;
  error: unknown;
  result: PlanApiResponse | undefined;
};

export function PlanForm({ onSubmit, isPending, isError, error, result }: Props) {
  const form = useForm({
    defaultValues: {
      region: "Temperate" as Region,
      rows: 4,
      cols: 4,
      periodStart: "",
      periodEnd: "",
      sun: "",
      soil: "",
      level: "",
      preferences: "",
      exclusions: "",
    },
    onSubmit: async ({ value }) => {
      const layout:PlanRequestLayout = Array.from({ length: value.rows }, () =>
        Array.from({ length: value.cols }, () => ({ type: "Empty" as const })),
      );

      const body: PlanRequest = {
        region: value.region,
        layout,
        ...(value.periodStart && value.periodEnd
          ? { period: { start: value.periodStart, end: value.periodEnd } }
          : {}),
        ...(value.sun ? { sun: value.sun as SunExposure } : {}),
        ...(value.soil ? { soil: value.soil as SoilType } : {}),
        ...(value.level ? { level: value.level as Level} : {}),
        preferences: value.preferences
          ? value.preferences
              .split(",")
              .map((s) => s.trim())
              .filter(Boolean)
              .map((id) => ({ id }))
          : null,
        exclusions: value.exclusions
          ? value.exclusions
              .split(",")
              .map((s) => s.trim())
              .filter(Boolean)
          : [],
      };

      onSubmit(body);
    },
  });

  return (
    <div className="space-y-6">
      <h1 className="text-2xl font-bold">Plan your garden</h1>

      <form
        onSubmit={(e) => {
          e.preventDefault();
          e.stopPropagation();
          void form.handleSubmit();
        }}
        className="space-y-4"
        aria-label="Garden plan form"
      >
        <Card>
          <CardHeader>
            <CardTitle className="text-base">Grid dimensions</CardTitle>
          </CardHeader>
          <CardContent className="grid gap-4 sm:grid-cols-2">
            <form.Field name="rows">
              {(field) => (
                <div className="space-y-1">
                  <Label htmlFor="rows">Rows</Label>
                  <Input
                    id="rows"
                    type="number"
                    min={1}
                    max={20}
                    value={field.state.value}
                    onChange={(e) => field.handleChange(Number(e.target.value))}
                  />
                </div>
              )}
            </form.Field>
            <form.Field name="cols">
              {(field) => (
                <div className="space-y-1">
                  <Label htmlFor="cols">Columns</Label>
                  <Input
                    id="cols"
                    type="number"
                    min={1}
                    max={20}
                    value={field.state.value}
                    onChange={(e) => field.handleChange(Number(e.target.value))}
                  />
                </div>
              )}
            </form.Field>
          </CardContent>
        </Card>

        <Card>
          <CardHeader>
            <CardTitle className="text-base">Constraints</CardTitle>
          </CardHeader>
          <CardContent className="grid gap-4 sm:grid-cols-2">
            <form.Field name="region">
              {(field) => (
                <div className="space-y-1">
                  <Label>Region *</Label>
                  <Select
                    value={field.state.value}
                    onValueChange={(v) => field.handleChange(v as PlanRequest["region"])}
                  >
                    <SelectTrigger aria-label="Region">
                      <SelectValue />
                    </SelectTrigger>
                    <SelectContent>
                      {REGIONS.map((r) => (
                        <SelectItem key={r} value={r}>
                          {r}
                        </SelectItem>
                      ))}
                    </SelectContent>
                  </Select>
                </div>
              )}
            </form.Field>

            <form.Field name="level">
              {(field) => (
                <div className="space-y-1">
                  <Label>Skill level</Label>
                  <Select
                    value={field.state.value || "_any"}
                    onValueChange={(v) => field.handleChange(v === "_any" ? "" : v)}
                  >
                    <SelectTrigger aria-label="Skill level">
                      <SelectValue placeholder="Any" />
                    </SelectTrigger>
                    <SelectContent>
                      <SelectItem value="_any">Any</SelectItem>
                      {LEVEL_OPTIONS.map((l) => (
                        <SelectItem key={l} value={l}>
                          {l}
                        </SelectItem>
                      ))}
                    </SelectContent>
                  </Select>
                </div>
              )}
            </form.Field>

            <form.Field name="sun">
              {(field) => (
                <div className="space-y-1">
                  <Label>Sun exposure</Label>
                  <Select
                    value={field.state.value || "_any"}
                    onValueChange={(v) => field.handleChange(v === "_any" ? "" : v)}
                  >
                    <SelectTrigger aria-label="Sun exposure">
                      <SelectValue placeholder="Any" />
                    </SelectTrigger>
                    <SelectContent>
                      <SelectItem value="_any">Any</SelectItem>
                      {SUN_OPTIONS.map((s) => (
                        <SelectItem key={s} value={s}>
                          {s}
                        </SelectItem>
                      ))}
                    </SelectContent>
                  </Select>
                </div>
              )}
            </form.Field>

            <form.Field name="soil">
              {(field) => (
                <div className="space-y-1">
                  <Label>Soil type</Label>
                  <Select
                    value={field.state.value || "_any"}
                    onValueChange={(v) => field.handleChange(v === "_any" ? "" : v)}
                  >
                    <SelectTrigger aria-label="Soil type">
                      <SelectValue placeholder="Any" />
                    </SelectTrigger>
                    <SelectContent>
                      <SelectItem value="_any">Any</SelectItem>
                      {SOIL_OPTIONS.map((s) => (
                        <SelectItem key={s} value={s}>
                          {s}
                        </SelectItem>
                      ))}
                    </SelectContent>
                  </Select>
                </div>
              )}
            </form.Field>
          </CardContent>
        </Card>

        <Card>
          <CardHeader>
            <CardTitle className="text-base">Period</CardTitle>
          </CardHeader>
          <CardContent className="grid gap-4 sm:grid-cols-2">
            <form.Field name="periodStart">
              {(field) => (
                <div className="space-y-1">
                  <Label htmlFor="periodStart">Start date</Label>
                  <Input
                    id="periodStart"
                    type="date"
                    value={field.state.value}
                    onChange={(e) => field.handleChange(e.target.value)}
                  />
                </div>
              )}
            </form.Field>
            <form.Field name="periodEnd">
              {(field) => (
                <div className="space-y-1">
                  <Label htmlFor="periodEnd">End date</Label>
                  <Input
                    id="periodEnd"
                    type="date"
                    value={field.state.value}
                    onChange={(e) => field.handleChange(e.target.value)}
                  />
                </div>
              )}
            </form.Field>
          </CardContent>
        </Card>

        <Card>
          <CardHeader>
            <CardTitle className="text-base">Preferences</CardTitle>
          </CardHeader>
          <CardContent className="space-y-4">
            <form.Field name="preferences">
              {(field) => (
                <div className="space-y-1">
                  <Label htmlFor="preferences">Preferred vegetable IDs (comma-separated)</Label>
                  <Input
                    id="preferences"
                    placeholder="e.g. tomato, basil, carrot"
                    value={field.state.value}
                    onChange={(e) => field.handleChange(e.target.value)}
                  />
                </div>
              )}
            </form.Field>
            <form.Field name="exclusions">
              {(field) => (
                <div className="space-y-1">
                  <Label htmlFor="exclusions">Exclusions (comma-separated)</Label>
                  <Input
                    id="exclusions"
                    placeholder="e.g. fennel"
                    value={field.state.value}
                    onChange={(e) => field.handleChange(e.target.value)}
                  />
                </div>
              )}
            </form.Field>
          </CardContent>
        </Card>

        <Button type="submit" disabled={isPending}>
          {isPending ? "Planning…" : "Generate plan"}
        </Button>

        {isError && (
          <p className="text-destructive text-sm" role="alert">
            Error: {String(error)}
          </p>
        )}
      </form>

      {result && (
        <div className="space-y-4">
          <h2 className="text-xl font-semibold">Results</h2>
          {result.payload.warnings.length > 0 && (
            <ul className="text-muted-foreground space-y-1 text-sm">
              {result.payload.warnings.map((w, i) => (
                <li key={i}>⚠ {w}</li>
              ))}
            </ul>
          )}
          <div className="space-y-6">
            {result.payload.weeks.map((week) => (
              <WeekGrid key={week.weekCount} week={week} />
            ))}
          </div>
        </div>
      )}
    </div>
  );
}
