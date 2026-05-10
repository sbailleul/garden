import { screen } from "@testing-library/react";

export const varietyDetailPage = {
  varietyName: (name: string) => screen.getByText(name),
  latinName: (name: string) => screen.getByText(name),
  queryLatinName: (name: string) => screen.queryByText(name),
  companionsLink: () => screen.getByText(/view companions/i),
};
