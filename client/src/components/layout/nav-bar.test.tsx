import { render, screen } from "@testing-library/react";
import { createMemoryHistory, createRouter, RouterProvider } from "@tanstack/react-router";
import { QueryClientProvider } from "@tanstack/react-query";
import { describe, expect, it } from "vitest";

import { queryClient } from "@/lib/query-client";
import { routeTree } from "@/routeTree.gen";

function renderAtPath(path: string) {
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

describe("NavBar", () => {
  it("renders the Vegetables link", async () => {
    renderAtPath("/vegetables");
    expect(await screen.findByRole("link", { name: /vegetables/i })).toBeInTheDocument();
  });

  it("renders the Plan Garden link", async () => {
    renderAtPath("/plan");
    expect(await screen.findByRole("link", { name: /plan garden/i })).toBeInTheDocument();
  });
});
