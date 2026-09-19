import { createBaseClient } from "@laffer/sdk";
import { publicActions, signerActions } from "../actions/index.js";

import type { Client, Transport } from "@laffer/sdk/types";

import type { SignerClient, SignerClientConfig } from "../types/clients.js";

export function createSignerClient<transport extends Transport = Transport>(
  parameters: SignerClientConfig<transport>,
): SignerClient<transport> {
  const { name = "Velox Signer Client", type = "velox" } = parameters;

  const client = createBaseClient({
    ...parameters,
    name,
    type,
  }) as unknown as Client<transport>;

  const publicClient = client.extend(publicActions) as unknown as SignerClient<transport>;
  return publicClient.extend(signerActions) as SignerClient<transport>;
}
