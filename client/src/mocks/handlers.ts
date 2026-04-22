import { http, HttpResponse } from "msw";

import type {
  Variety,
  Vegetable,
  VegetablesApiResponse,
  VegetableApiResponse,
  VarietiesApiResponse,
  VarietyApiResponse,
  CompanionsApiResponse,
  PlanApiResponse,
} from "@/api/types";

const TOMATO: Variety = {
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

const BASIL: Variety = {
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

const varietyResponse = (v: Variety): VarietyApiResponse => ({
  payload: v as VarietyApiResponse["payload"],
  _links: {
    self: SELF_LINK(`/api/varieties/${v.id}`),
    companions: SELF_LINK(`/api/varieties/${v.id}/companions`),
    collection: SELF_LINK("/api/varieties"),
  },
});

const VEGETABLES_RESPONSE: VarietiesApiResponse = {
  payload: [TOMATO, BASIL].map(varietyResponse),
  pagination: { page: 1, perPage: 100, total: 2, totalPages: 1 },
  _links: {
    self: SELF_LINK("/api/varieties"),
  },
};

const COMPANIONS_RESPONSE: CompanionsApiResponse = {
  payload: {
    id: "tomato",
    name: "Tomato",
    good: [{ id: "basil", name: "Basil" }],
    bad: [{ id: "fennel", name: "Fennel" }],
  },
  _links: {
    self: SELF_LINK("/api/varieties/tomato/companions"),
    variety: SELF_LINK("/api/varieties/tomato"),
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
              plantsPerCell: 1,
              estimatedHarvestDate: "2025-08-11",
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
    varieties: SELF_LINK("/api/varieties"),
  },
};

const VEGETABLE_MAP: Record<string, Variety> = {
  tomato: TOMATO,
  basil: BASIL,
};

const TOMATO_VARIETY: Vegetable = { id: "tomato", name: "Tomato" };
const BASIL_VARIETY: Vegetable = { id: "basil", name: "Basil" };

const vegetableResponse = (v: Vegetable): VegetableApiResponse => ({
  payload: v,
  _links: {
    self: SELF_LINK(`/api/vegetables/${v.id}`),
    collection: SELF_LINK("/api/vegetables"),
  },
});

const VARIETIES_RESPONSE: VegetablesApiResponse = {
  payload: [TOMATO_VARIETY, BASIL_VARIETY].map(vegetableResponse),
  pagination: { page: 1, perPage: 100, total: 2, totalPages: 1 },
  _links: { self: SELF_LINK("/api/vegetables") },
};

const VARIETY_MAP: Record<string, Vegetable> = {
  tomato: TOMATO_VARIETY,
  basil: BASIL_VARIETY,
};

export const handlers = [
  http.get("/api/varieties", () => HttpResponse.json(VEGETABLES_RESPONSE)),

  http.get("/api/varieties/:id", ({ params }) => {
    const veg = VEGETABLE_MAP[params['id']as string];
    if (!veg) {
      return HttpResponse.json({ error: "Not found" }, { status: 404 });
    }
    return HttpResponse.json(varietyResponse(veg));
  }),

  http.get("/api/varieties/:id/companions", ({ params }) => {
    if (params['id'] !== "tomato") {
      return HttpResponse.json({ error: "Not found" }, { status: 404 });
    }
    return HttpResponse.json(COMPANIONS_RESPONSE);
  }),

  http.post("/api/plan", () => HttpResponse.json(PLAN_RESPONSE)),

  http.get("/api/vegetables", () => HttpResponse.json(VARIETIES_RESPONSE)),

  http.get("/api/vegetables/:id", ({ params }) => {
    const vegetable = VARIETY_MAP[params["id"] as string];
    if (!vegetable) {
      return HttpResponse.json({ error: "Not found" }, { status: 404 });
    }
    return HttpResponse.json(vegetableResponse(vegetable));
  }),
];
