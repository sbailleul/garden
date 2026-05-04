import { http, HttpResponse } from "msw";

import type {
  Group,
  GroupApiResponse,
  GroupsApiResponse,
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
  vegetableId: "tomato",
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
  vegetableId: "basil",
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
    companions: SELF_LINK(`/api/vegetables/${v.vegetableId}/companions`),
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

const TOMATO_VARIETY: Vegetable = { id: "tomato", name: "Tomato", groupId: "legumes-fruits", varietyIds: ["tomato"], goodCompanions: ["basil"], badCompanions: ["fennel"] };
const BASIL_VARIETY: Vegetable = { id: "basil", name: "Basil", groupId: "legumes-feuilles", varietyIds: ["basil"], goodCompanions: ["tomato"], badCompanions: [] };

const vegetableResponse = (v: Vegetable): VegetableApiResponse => ({
  payload: v,
  _links: {
    self: SELF_LINK(`/api/vegetables/${v.id}`),
    companions: SELF_LINK(`/api/vegetables/${v.id}/companions`),
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

const BULBES_GROUP: Group = { id: "bulbes", name: "Bulbes" };
const LEGUMES_FRUITS_GROUP: Group = { id: "legumes-fruits", name: "Légumes-Fruits" };
const LEGUMES_FEUILLES_GROUP: Group = { id: "legumes-feuilles", name: "Légumes-Feuilles" };
const LEGUMES_RACINES_GROUP: Group = { id: "legumes-racines", name: "Légumes-Racines" };
const ENGRAIS_VERTS_GROUP: Group = { id: "engrais-verts", name: "Engrais Verts" };
const PLANTES_A_GRAINS_GROUP: Group = { id: "plantes-a-grains", name: "Plantes à Grains" };

const ALL_GROUPS = [
  BULBES_GROUP,
  LEGUMES_FEUILLES_GROUP,
  LEGUMES_FRUITS_GROUP,
  LEGUMES_RACINES_GROUP,
  ENGRAIS_VERTS_GROUP,
  PLANTES_A_GRAINS_GROUP,
];

const groupResponse = (g: Group): GroupApiResponse => ({
  payload: g,
  _links: {
    self: SELF_LINK(`/api/groups/${g.id}`),
    vegetables: SELF_LINK(`/api/groups/${g.id}/vegetables`),
    collection: SELF_LINK("/api/groups"),
  },
});

const GROUPS_RESPONSE: GroupsApiResponse = {
  payload: ALL_GROUPS.map(groupResponse),
  pagination: { page: 1, perPage: 20, total: 6, totalPages: 1 },
  _links: { self: SELF_LINK("/api/groups") },
};

const GROUP_MAP: Record<string, Group> = Object.fromEntries(
  ALL_GROUPS.map((g) => [g.id, g]),
);

const GROUP_VEGETABLES: Record<string, Vegetable[]> = {
  "legumes-fruits": [TOMATO_VARIETY],
  "legumes-feuilles": [BASIL_VARIETY],
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

  http.get("/api/vegetables/:id/companions", ({ params }) => {
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

  http.get("/api/groups", () => HttpResponse.json(GROUPS_RESPONSE)),

  http.get("/api/groups/:id", ({ params }) => {
    const group = GROUP_MAP[params["id"] as string];
    if (!group) {
      return HttpResponse.json({ error: "Not found" }, { status: 404 });
    }
    return HttpResponse.json(groupResponse(group));
  }),

  http.get("/api/groups/:id/vegetables", ({ params }) => {
    const group = GROUP_MAP[params["id"] as string];
    if (!group) {
      return HttpResponse.json({ error: "Not found" }, { status: 404 });
    }
    const vegetables = GROUP_VEGETABLES[params["id"] as string] ?? [];
    const response: VegetablesApiResponse = {
      payload: vegetables.map(vegetableResponse),
      pagination: {
        page: 1,
        perPage: 20,
        total: vegetables.length,
        totalPages: 1,
      },
      _links: {
        self: SELF_LINK(`/api/groups/${params["id"]}/vegetables`),
        group: SELF_LINK(`/api/groups/${params["id"]}`),
      },
    };
    return HttpResponse.json(response);
  }),

  http.get("/api/vegetables/:id/varieties", ({ params }) => {
    const vegetableId = params["id"] as string;
    const vegetable = VARIETY_MAP[vegetableId];
    if (!vegetable) {
      return HttpResponse.json({ error: "Not found" }, { status: 404 });
    }
    // Return the mock varieties that belong to this vegetable
    const matching = Object.values(VEGETABLE_MAP).filter(
      (v) => v.vegetableId === vegetableId,
    );
    const response: VarietiesApiResponse = {
      payload: matching.map(varietyResponse),
      pagination: {
        page: 1,
        perPage: 20,
        total: matching.length,
        totalPages: 1,
      },
      _links: {
        self: SELF_LINK(`/api/vegetables/${vegetableId}/varieties`),
        vegetable: SELF_LINK(`/api/vegetables/${vegetableId}`),
      },
    };
    return HttpResponse.json(response);
  }),
];
