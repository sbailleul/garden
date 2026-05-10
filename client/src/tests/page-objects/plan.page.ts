import { screen } from "@testing-library/react";

export const planPage = {
  generateButton: () => screen.getByRole("button", { name: /generate plan/i }),
  generateButtons: () => screen.getAllByRole("button", { name: /generate plan/i }),
  weekGrid: (week = 1) =>
    screen.getByRole("grid", { name: new RegExp(`week ${week} garden grid`, "i") }),
};
