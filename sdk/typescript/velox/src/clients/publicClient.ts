import { createBaseClient } from "@laffer/sdk";
import { publicActions } from "../actions/index.js";

import type { Client, Transport } from "@laffer/sdk/types";
import type { PublicClient, PublicClientConfig } from "../types/clients.js";

export function createPublicClient<transport extends Transport>(
  parameters: PublicClientConfig<transport>,
): PublicClient<transport> {
  const { name = "Velox Public Client" } = parameters;

  const client = createBaseClient({
    ...parameters,
    name,
    type: "velox",
  }) as Client<transport>;

  return client.extend(publicActions) as PublicClient<transport>;
}
