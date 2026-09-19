import { publicActions } from "@laffer/velox";

import type { PublicClient } from "@laffer/velox/types";

import type { Config } from "../types/index.js";

export type GetPublicClientReturnType = PublicClient;

export type GetPublicClientErrorType = Error;

export function getPublicClient<config extends Config>(config: config): GetPublicClientReturnType {
  const client = config.getClient();
  return client.extend(publicActions) as unknown as PublicClient;
}
