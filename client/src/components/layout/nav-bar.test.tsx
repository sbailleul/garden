import { describe, expect, it } from "vitest";

import { navBarPage } from "@/tests/page-objects/nav-bar.page";
import { renderAt } from "@/tests/render-at";

describe("NavBar", () => {
  it("renders the Varieties link", async () => {
    renderAt("/varieties");
    expect(await navBarPage.varietiesLink()).toBeInTheDocument();
  });

  it("renders the Plan Garden link", async () => {
    renderAt("/plan");
    expect(await navBarPage.planGardenLink()).toBeInTheDocument();
  });
});
