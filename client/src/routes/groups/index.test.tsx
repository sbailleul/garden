import { waitFor } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import { groupsPage } from "@/tests/page-objects/groups.page";
import { renderAt } from "@/tests/render-at";

describe("Group catalogue", () => {
  it("renders group names from MSW fixture", async () => {
    renderAt("/groups");

    await waitFor(() => {
      expect(groupsPage.groupName("Bulbes")).toBeInTheDocument();
      expect(groupsPage.groupName("Légumes-Fruits")).toBeInTheDocument();
    });
  });

  it("shows group count below table", async () => {
    renderAt("/groups");

    await waitFor(() => {
      expect(groupsPage.groupCount()).toBeInTheDocument();
    });
  });
});
