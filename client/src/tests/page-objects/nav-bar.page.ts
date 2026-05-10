import { screen } from "@testing-library/react";

export const navBarPage = {
  varietiesLink: () => screen.findByRole("link", { name: /varieties/i }),
  planGardenLink: () => screen.findByRole("link", { name: /plan garden/i }),
};
