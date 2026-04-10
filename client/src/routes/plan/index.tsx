import { createFileRoute } from "@tanstack/react-router";
import { useMutation } from "@tanstack/react-query";

import { postPlan } from "@/api/plan";
import type { PlanRequest } from "@/api/types";
import { PlanForm } from "@/components/plan/form";

export const Route = createFileRoute("/plan/")({
  component: PlannerPage,
});

function PlannerPage() {
  const mutation = useMutation({ mutationFn: postPlan });

  return (
    <PlanForm
      onSubmit={(body: PlanRequest) => {
        void mutation.mutateAsync(body);
      }}
      isPending={mutation.isPending}
      isError={mutation.isError}
      error={mutation.error}
      result={mutation.data}
    />
  );
}

