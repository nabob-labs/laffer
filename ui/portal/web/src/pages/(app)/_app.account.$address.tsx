import { createFileRoute } from "@tanstack/react-router";
import { m } from "@laffer/foundation/paraglide/messages.js";

export const Route = createFileRoute("/(app)/_app/account/$address")({
  head: () => ({
    meta: [{ title: `Velox | ${m["explorer.accounts.title"]()}` }],
  }),
});
