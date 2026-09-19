import { createFileRoute } from "@tanstack/react-router";
import { m } from "@laffer/foundation/paraglide/messages.js";

import { Landing } from "~/components/landing/Landing";

export const Route = createFileRoute("/(app)/_app/")({
  head: () => ({
    meta: [{ title: `Velox | ${m["common.overview"]()}` }],
  }),
  component: OverviewComponent,
});

function OverviewComponent() {
  return <Landing />;
}
