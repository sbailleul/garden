import { screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";

export const varietiesPage = {
  varietyName: (name: string) => screen.getByText(name),
  allByName: (name: string) => screen.getAllByText(name),
  queryVarietyName: (name: string) => screen.queryByText(name),
  varietyCount: () => screen.getByText(/\d+ varieties/i),
  searchInput: () => screen.getByRole("textbox", { name: /filter by name/i }),
  async typeSearch(text: string) {
    await userEvent.type(varietiesPage.searchInput(), text);
  },
};
