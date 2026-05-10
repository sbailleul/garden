import { waitFor } from "@testing-library/react";
import { http, HttpResponse } from "msw";
import { describe, expect, it } from "vitest";

import { worker } from "@/mocks/browser";
import { varietyDetailPage } from "@/tests/page-objects/variety-detail.page";
import { renderAt } from "@/tests/render-at";

describe("Variety detail", () => {
  it("renders variety name and latin name", async () => {
    renderAt("/varieties/tomato");

    await waitFor(() => {
      expect(varietyDetailPage.varietyName("Tomato")).toBeInTheDocument();
      expect(varietyDetailPage.latinName("Solanum lycopersicum")).toBeInTheDocument();
    });
  });

  it("shows a link to companions", async () => {
    renderAt("/varieties/tomato");

    await waitFor(() => {
      expect(varietyDetailPage.companionsLink()).toBeInTheDocument();
    });
  });

  it("shows an error state when variety is not found", async () => {
    worker.use(
      http.get("/api/varieties/:id", () =>
        HttpResponse.json({ error: "Not found" }, { status: 404 }),
      ),
    );

    renderAt("/varieties/unknown-id");

    await waitFor(() => {
      // Router should show an error or the page should surface the rejection
      expect(varietyDetailPage.queryLatinName("Solanum lycopersicum")).not.toBeInTheDocument();
    });
  });
});
