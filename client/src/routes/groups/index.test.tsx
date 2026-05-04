import { render, screen, waitFor } from "@testing-library/react";
import {
  createMemoryHistory,
  createRouter,
  RouterProvider,
} from "@tanstack/react-router";
import { QueryClientProvider } from "@tanstack/react-query";
import { describe, expect, it } from "vitest";

import { queryClient } from "@/lib/query-client";
import { routeTree } from "@/routeTree.gen";

function renderAt(path: string) {
  queryClient.clear();
  const history = createMemoryHistory({ initialEntries: [path] });
  const router = createRouter({
    routeTree,
    history,
    context: { queryClient },
  });
  return render(
    <QueryClientProvider client={queryClient}>
      <RouterProvider router={router} />
    </QueryClientProvider>,
  );
}

describe("Group catalogue", () => {
  it("renders group names from MSW fixture", async () => {
    renderAt("/groups");

    await waitFor(() => {
      expect(screen.getByText("Bulbes")).toBeInTheDocument();
      expect(screen.getByText("Légumes-Fruits")).toBeInTheDocument();
    });
  });

  it("shows group count below table", async () => {
    renderAt("/groups");

    await waitFor(() => {
      expect(screen.getByText(/6 groups/i)).toBeInTheDocument();
    });
  });
});
