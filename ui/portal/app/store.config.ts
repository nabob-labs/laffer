import { createConfig, graphql, passkey, devnet, createStorage } from "@laffer/store";

import { createMMKVStorage } from "./storage.config";
import { coins } from "@laffer/foundation/coins";

import type { Config } from "@laffer/store/types";

const chain = devnet;

export const config: Config = createConfig({
  multiInjectedProviderDiscovery: false,
  chain,
  transport: graphql(chain.urls.indexer, { batch: true }),
  coins,
  storage: createStorage({ storage: createMMKVStorage() }),
  connectors: [passkey()],
});
