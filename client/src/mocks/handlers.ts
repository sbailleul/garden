import { http, HttpResponse } from "msw";

import type { components } from "@/api/schema.d.ts";

type Vegetable = components["schemas"]["Vegetable"];
type VegetablesApiResponse = components["schemas"]["VegetablesApiResponse"];
type VegetableApiResponse = components["schemas"]["VegetableApiResponse"];
type CompanionsApiResponse = components["schemas"]["CompanionsApiResponse"];
type PlanApiResponse = components["schemas"]["PlanApiResponse"];

const TOMATO: Vegetable = {
  id: "tomato",
  name: "Tomato",
  latinName: "Solanum lycopersicum",
  category: "Fruit",
  lifecycle: "Annual",
  spacingCm: 60,
  daysToHarvest: 70,
  daysToPlant: 42,
  beginnerFriendly: true,
  soilTypes: ["Loamy"],
  sunRequirement: ["FullSun"],
  goodCompanions: ["basil"],
  badCompanions: ["fennel"],
  calendars: [
    {
      region: "Temperate",
      sowing: { indoor: ["February", "March"], outdoor: [] },
      planting: { indoor: [], outdoor: ["May", "June"] },
    },
  ],
};

const BASIL: Vegetable = {
  id: "basil",
  name: "Basil",
  latinName: "Ocimum basilicum",
  category: "Herb",
  lifecycle: "Annual",
  spacingCm: 30,
  daysToHarvest: 30,
  daysToPlant: 14,
  beginnerFriendly: true,
  soilTypes: ["Loamy"],
  sunRequirement: ["FullSun"],
  goodCompanions: ["tomato"],
  badCompanions: [],
  calendars: [
    {
      region: "Temperate",
      sowing: { indoor: ["April"], outdoor: ["May"] },
      planting: { indoor: [], outdoor: ["May", "June"] },
    },
  ],
};

const SELF_LINK = (path: string) => ({ href: path, method: "GET" });

const VEGETABLES_RESPONSE: VegetablesApiResponse = {
  payload: [TOMATO, BASIL] as VegetablesApiResponse["payload"],
  pagination: { page: 1, perPage: 100, total: 2, totalPages: 1 },
  _links: {
    self: SELF_LINK("/api/vegetables"),
  },
};

const vegetableResponse = (v: Vegetable): VegetableApiResponse => ({
  payload: v as VegetableApiResponse["payload"],
  _links: {
    self: SELF_LINK(`/api/vegetables/${v.id}`),
    companions: SELF_LINK(`/api/vegetables/${v.id}/companions`),
    collection: SELF_LINK("/api/vegetables"),
  },
});

const COMPANIONS_RESPONSE: CompanionsApiResponse = {
  payload: {
    id: "tomato",
    name: "Tomato",
    good: [{ id: "basil", name: "Basil" }],
    bad: [{ id: "fennel", name: "Fennel" }],
  },
  _links: {
    self: SELF_LINK("/api/vegetables/tomato/companions"),
    vegetable: SELF_LINK("/api/vegetables/tomato"),
  },
};

const PLAN_RESPONSE: PlanApiResponse = {
  payload: {
    rows: 2,
    cols: 2,
    weeks: [
      {
        weekCount: 1,
        period: { start: "2025-06-02", end: "2025-06-08" },
        score: 10,
        sowingTasks: [],
        grid: [
          [
            {
              type: "SelfContained",
              id: "tomato",
              name: "Tomato",
              reason: "Auto-placed",
              plants_per_cell: 1,
              estimated_harvest_date: "2025-08-11",
            },
            { type: "Empty" },
          ],
          [{ type: "Empty" }, { type: "Empty" }],
        ],
      },
    ],
    warnings: [],
  },
  _links: {
    self: { href: "/api/plan", method: "POST" },
    vegetables: SELF_LINK("/api/vegetables"),
  },
};

const VEGETABLE_MAP: Record<string, Vegetable> = {
  tomato: TOMATO,
  basil: BASIL,
};

export const handlers = [
  http.get("/api/vegetables", () => HttpResponse.json(VEGETABLES_RESPONSE)),

  http.get("/api/vegetables/:id", ({ params }) => {
    const veg = VEGETABLE_MAP[params.id as string];
    if (!veg) {
      return HttpResponse.json({ error: "Not found" }, { status: 404 });
    }
    return HttpResponse.json(vegetableResponse(veg));
  }),

  http.get("/api/vegetables/:id/companions", ({ params }) => {
    if (params.id !== "tomato") {
      return HttpResponse.json({ error: "Not found" }, { status: 404 });
    }
    return HttpResponse.json(COMPANIONS_RESPONSE);
  }),

  http.post("/api/plan", () => HttpResponse.json(PLAN_RESPONSE)),
];
