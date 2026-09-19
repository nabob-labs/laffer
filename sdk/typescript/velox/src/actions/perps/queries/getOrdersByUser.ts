import { queryWasmSmart } from "@laffer/sdk";
import type { Client, Prettify, Transport } from "@laffer/sdk/types";

import { getAction, getAppConfig } from "@laffer/sdk/actions";
import type { Chain, Signer } from "@laffer/sdk/types";
import type { AppConfig } from "../../../types/app.js";
import type { GetPerpsQueryMsg, PerpsOrdersByUserResponse } from "../../../types/perps.js";

type ActionMsg = GetPerpsQueryMsg<"ordersByUser">;

export type GetPerpsOrdersByUserParameters = Prettify<
  ActionMsg["ordersByUser"] & { height?: number }
>;

export type GetPerpsOrdersByUserReturnType = Promise<PerpsOrdersByUserResponse>;

export async function getPerpsOrdersByUser<
  chain extends Chain | undefined,
  signer extends Signer | undefined,
>(
  client: Client<Transport, chain, signer>,
  parameters: GetPerpsOrdersByUserParameters,
): GetPerpsOrdersByUserReturnType {
  const { height = 0, ...queryMsg } = parameters;

  const action = getAction(client, getAppConfig, "getAppConfig");

  const msg: ActionMsg = {
    ordersByUser: {
      ...queryMsg,
    },
  };

  const { addresses } = await action<AppConfig>({});

  return await queryWasmSmart(client, { contract: addresses.perps, msg, height });
}
