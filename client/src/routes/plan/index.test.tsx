import { waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it } from "vitest";

import { planPage } from "@/tests/page-objects/plan.page";
import { renderAt } from "@/tests/render-at";

describe("Plan form", () => {
  it("renders the plan form with a submit button", async () => {
    renderAt("/plan");

    await waitFor(() => {
      expect(planPage.generateButton()).toBeInTheDocument();
    });
  });

  it("shows grid after submitting the plan", async () => {
    const user = userEvent.setup();
    renderAt("/plan");

    await waitFor(() => planPage.generateButtons());

    await user.click(planPage.generateButtons()[0]!);

    await waitFor(() => {
      // MSW mock returns a 2×2 grid with one Tomato cell
      expect(planPage.weekGrid()).toBeInTheDocument();
    });
  });
});
