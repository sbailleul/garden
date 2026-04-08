import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { createMemoryHistory, createRouter, RouterProvider } from "@tanstack/react-router";
import { QueryClientProvider } from "@tanstack/react-query";
import { describe, expect, it } from "vitest";

import { queryClient } from "../lib/queryClient";
import { routeTree } from "../routeTree.gen";

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

describe("Plan form", () => {
  it("renders the plan form with a submit button", async () => {
    renderAt("/plan");

    await waitFor(() => {
      expect(screen.getByRole("button", { name: /generate plan/i })).toBeInTheDocument();
    });
  });

  it("shows grid after submitting the plan", async () => {
    const user = userEvent.setup();
    renderAt("/plan");

    await waitFor(() => screen.getAllByRole("button", { name: /generate plan/i }));

    await user.click(screen.getAllByRole("button", { name: /generate plan/i })[0]);

    await waitFor(() => {
      // MSW mock returns a 2×2 grid with one Tomato cell
      expect(screen.getByRole("grid", { name: /week 1 garden grid/i })).toBeInTheDocument();
    });
  });
});
