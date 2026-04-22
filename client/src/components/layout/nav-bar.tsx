import { Link, useRouterState } from "@tanstack/react-router";

const LINKS = [
  { to: "/varieties", label: "Varieties" },
  { to: "/plan", label: "Plan Garden" },
] as const;

export function NavBar() {
  const { location } = useRouterState();

  return (
    <nav className="border-border bg-card border-b px-4 py-3">
      <div className="mx-auto flex max-w-5xl items-center gap-6">
        <span className="text-primary font-semibold tracking-tight">🌱 Garden Planner</span>
        <div className="flex gap-4">
          {LINKS.map(({ to, label }) => (
            <Link
              key={to}
              to={to}
              className={[
                "text-sm font-medium transition-colors",
                location.pathname.startsWith(to)
                  ? "text-primary"
                  : "text-muted-foreground hover:text-foreground",
              ].join(" ")}
            >
              {label}
            </Link>
          ))}
        </div>
      </div>
    </nav>
  );
}
