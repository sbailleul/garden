import { Outlet } from "@tanstack/react-router";

import { NavBar } from "./NavBar";

export function AppLayout() {
  return (
    <div className="bg-background text-foreground min-h-screen">
      <NavBar />
      <main className="mx-auto max-w-5xl px-4 py-6">
        <Outlet />
      </main>
    </div>
  );
}
