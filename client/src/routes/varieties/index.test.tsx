import { waitFor } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import { varietiesPage } from "@/tests/page-objects/varieties.page";
import { renderAt } from "@/tests/render-at";

describe("Variety catalogue", () => {
  it("renders a table with variety rows from MSW fixture", async () => {
    renderAt("/varieties");

    await waitFor(() => {
      expect(varietiesPage.varietyName("Tomato")).toBeInTheDocument();
      expect(varietiesPage.varietyName("Basil")).toBeInTheDocument();
    });
  });

  it("shows row count below table", async () => {
    renderAt("/varieties");

    await waitFor(() => {
      expect(varietiesPage.varietyCount()).toBeInTheDocument();
    });
  });

  it("filters rows by name", async () => {
    renderAt("/varieties");

    await waitFor(() => varietiesPage.allByName("Tomato").length > 0);

    await varietiesPage.typeSearch("bas");

    await waitFor(() => {
      expect(varietiesPage.varietyName("Basil")).toBeInTheDocument();
      expect(varietiesPage.queryVarietyName("Tomato")).not.toBeInTheDocument();
    });
  });
});
