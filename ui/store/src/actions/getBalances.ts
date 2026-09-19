import {
  type GetBalancesParameters as _GetBalancesParameters_,
  getBalances as getBalancesAction,
} from "@laffer/velox/actions";
import { getPublicClient } from "./getPublicClient.js";

import type { Coins, Prettify } from "@laffer/velox/types";
import type { Config } from "../types/store.js";

export type GetBalanceParameters = Prettify<_GetBalancesParameters_>;

export type GetBalancesReturnType = Coins;

export type GetBalancesErrorType = Error;

export async function getBalances<config extends Config>(
  config: config,
  parameters: GetBalanceParameters,
): Promise<GetBalancesReturnType> {
  const client = getPublicClient(config);
  return await getBalancesAction(client, parameters);
}
