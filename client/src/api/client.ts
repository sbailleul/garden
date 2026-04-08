import createClient from "openapi-fetch";

import type { paths } from "./schema.d.ts";

export const apiClient = createClient<paths>({
  baseUrl: import.meta.env.VITE_API_URL ?? "",
});
