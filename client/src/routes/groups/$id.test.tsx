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

describe("Group detail", () => {
  it("renders group name and id", async () => {
    renderAt("/groups/legumes-fruits");

    await waitFor(() => {
      expect(screen.getByText("Légumes-Fruits")).toBeInTheDocument();
      expect(screen.getByText("legumes-fruits")).toBeInTheDocument();
    });
  });

  it("renders vegetables belonging to the group", async () => {
    renderAt("/groups/legumes-fruits");

    await waitFor(() => {
      expect(screen.getByText("Tomato")).toBeInTheDocument();
    });
  });

  it("shows a link back to the groups list", async () => {
    renderAt("/groups/legumes-feuilles");

    await waitFor(() => {
      expect(screen.getByText(/← Groups/i)).toBeInTheDocument();
    });
  });
});
