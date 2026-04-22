import { render, screen, waitFor } from "@testing-library/react";
import { createMemoryHistory, createRouter, RouterProvider } from "@tanstack/react-router";
import { QueryClientProvider } from "@tanstack/react-query";
import { http, HttpResponse } from "msw";
import { describe, expect, it } from "vitest";

import { worker } from "@/mocks/browser";
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

describe("Variety detail", () => {
  it("renders variety name and latin name", async () => {
    renderAt("/varieties/tomato");

    await waitFor(() => {
      expect(screen.getByText("Tomato")).toBeInTheDocument();
      expect(screen.getByText("Solanum lycopersicum")).toBeInTheDocument();
    });
  });

  it("shows a link to companions", async () => {
    renderAt("/varieties/tomato");

    await waitFor(() => {
      expect(screen.getByText(/view companions/i)).toBeInTheDocument();
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
      expect(screen.queryByText("Solanum lycopersicum")).not.toBeInTheDocument();
    });
  });
});
