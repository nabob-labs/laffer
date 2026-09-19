import { createFileRoute } from "@tanstack/react-router";

import { m } from "@laffer/foundation/paraglide/messages.js";

export const Route = createFileRoute("/(app)/_app/settings")({
  head: () => ({
    meta: [{ title: `Velox | ${m["settings.title"]()}` }],
  }),
});
