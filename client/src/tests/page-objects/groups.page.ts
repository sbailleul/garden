import { screen } from "@testing-library/react";

export const groupsPage = {
  groupName: (name: string) => screen.getByText(name),
  groupCount: () => screen.getByText(/\d+ groups/i),
};
