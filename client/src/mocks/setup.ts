import "@testing-library/jest-dom/vitest";
import { cleanup } from "@testing-library/react";
import { afterAll, afterEach, beforeAll } from "vitest";

import { worker } from "./browser";

beforeAll(async () => {
  await worker.start({ onUnhandledRequest: "error" });
});

afterEach(() => {
  worker.resetHandlers();
  cleanup();
});

afterAll(() => {
  worker.stop();
});
