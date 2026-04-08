import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { createMemoryHistory, createRouter, RouterProvider } from "@tanstack/react-router";
import { QueryClientProvider } from "@tanstack/react-query";
import { describe, expect, it } from "vitest";

import { queryClient } from "../../lib/queryClient";
import { routeTree } from "../../routeTree.gen";

function createTestRouter(initialPath: string) {
  const history = createMemoryHistory({ initialEntries: [initialPath] });
  return createRouter({
    routeTree,
    history,
    context: { queryClient },
  });
}

function renderAt(path: string) {
  queryClient.clear();
  const router = createTestRouter(path);
  return render(
    <QueryClientProvider client={queryClient}>
      <RouterProvider router={router} />
    </QueryClientProvider>,
  );
}

describe("Vegetable catalogue", () => {
  it("renders a table with vegetable rows from MSW fixture", async () => {
    renderAt("/vegetables");

    await waitFor(() => {
      expect(screen.getByText("Tomato")).toBeInTheDocument();
      expect(screen.getByText("Basil")).toBeInTheDocument();
    });
  });

  it("shows row count below table", async () => {
    renderAt("/vegetables");

    await waitFor(() => {
      expect(screen.getByText(/2 vegetables/i)).toBeInTheDocument();
    });
  });

  it("filters rows by name", async () => {
    const user = userEvent.setup();
    renderAt("/vegetables");

    await waitFor(() => screen.getAllByText("Tomato").length > 0);

    const input = screen.getByRole("textbox", { name: /filter by name/i });
    await user.type(input, "bas");

    await waitFor(() => {
      expect(screen.getByText("Basil")).toBeInTheDocument();
      expect(screen.queryByText("Tomato")).not.toBeInTheDocument();
    });
  });
});
