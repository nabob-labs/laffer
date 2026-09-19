import { queryWasmSmart } from "@laffer/sdk";
import type { Client, Prettify, Transport } from "@laffer/sdk/types";

import { getAction, getAppConfig } from "@laffer/sdk/actions";
import type { Chain, Signer } from "@laffer/sdk/types";
import type { AppConfig } from "../../../types/app.js";
import type { GetPerpsQueryMsg, PerpsUserState } from "../../../types/perps.js";

type ActionMsg = GetPerpsQueryMsg<"userState">;

export type GetPerpsUserStateParameters = Prettify<ActionMsg["userState"] & { height?: number }>;

export type GetPerpsUserStateReturnType = Promise<PerpsUserState | null>;

export async function getPerpsUserState<
  chain extends Chain | undefined,
  signer extends Signer | undefined,
>(
  client: Client<Transport, chain, signer>,
  parameters: GetPerpsUserStateParameters,
): GetPerpsUserStateReturnType {
  const { height = 0, ...queryMsg } = parameters;

  const action = getAction(client, getAppConfig, "getAppConfig");

  const msg: ActionMsg = {
    userState: {
      ...queryMsg,
    },
  };

  const { addresses } = await action<AppConfig>({});

  return await queryWasmSmart(client, { contract: addresses.perps, msg, height });
}
