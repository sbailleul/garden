import { screen } from "@testing-library/react";

export const groupDetailPage = {
  groupName: (name: string) => screen.getByText(name),
  groupId: (id: string) => screen.getByText(id),
  vegetableName: (name: string) => screen.getByText(name),
  backLink: () => screen.getByText(/← Groups/i),
};
