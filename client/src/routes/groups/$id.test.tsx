import { waitFor } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import { groupDetailPage } from "@/tests/page-objects/group-detail.page";
import { renderAt } from "@/tests/render-at";

describe("Group detail", () => {
  it("renders group name and id", async () => {
    renderAt("/groups/legumes-fruits");

    await waitFor(() => {
      expect(groupDetailPage.groupName("Légumes-Fruits")).toBeInTheDocument();
      expect(groupDetailPage.groupId("legumes-fruits")).toBeInTheDocument();
    });
  });

  it("renders vegetables belonging to the group", async () => {
    renderAt("/groups/legumes-fruits");

    await waitFor(() => {
      expect(groupDetailPage.vegetableName("Tomato")).toBeInTheDocument();
    });
  });

  it("shows a link back to the groups list", async () => {
    renderAt("/groups/legumes-feuilles");

    await waitFor(() => {
      expect(groupDetailPage.backLink()).toBeInTheDocument();
    });
  });
});
