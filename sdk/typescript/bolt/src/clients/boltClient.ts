import type { Chain, Client, ClientConfig, Transport } from "../types/index.js";

import { type BoltActions, boltActions } from "../actions/boltActions.js";
import { createBaseClient } from "./baseClient.js";

export type BoltClientConfig<
  transport extends Transport = Transport,
  chain extends Chain | undefined = Chain | undefined,
> = ClientConfig<transport, chain, undefined>;

export type BoltClient<
  transport extends Transport = Transport,
  chain extends Chain | undefined = Chain | undefined,
> = Client<transport, chain, undefined, BoltActions<transport, chain>>;

export function createBoltClient<
  transport extends Transport,
  chain extends Chain | undefined = undefined,
>(parameters: BoltClientConfig<transport, chain>): BoltClient<transport, chain> {
  const { name = "Bolt Client" } = parameters;
  const client = createBaseClient({
    ...parameters,
    name,
    type: "bolt",
  });
  return client.extend(boltActions);
}
