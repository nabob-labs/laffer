import { queryWasmSmart } from "@laffer/sdk";
import type { Client, Prettify, Transport } from "@laffer/sdk/types";

import { getAction, getAppConfig } from "@laffer/sdk/actions";
import type { Chain, Signer } from "@laffer/sdk/types";
import type { AppConfig } from "../../../types/app.js";
import type { GetPerpsQueryMsg, PerpsParam } from "../../../types/perps.js";

type ActionMsg = GetPerpsQueryMsg<"param">;

export type GetPerpsParamParameters = Prettify<{ height?: number }>;

export type GetPerpsParamReturnType = Promise<PerpsParam>;

export async function getPerpsParam<
  chain extends Chain | undefined,
  signer extends Signer | undefined,
>(
  client: Client<Transport, chain, signer>,
  parameters?: GetPerpsParamParameters,
): GetPerpsParamReturnType {
  const { height = 0 } = parameters ?? {};

  const action = getAction(client, getAppConfig, "getAppConfig");

  const msg: ActionMsg = {
    param: {},
  };

  const { addresses } = await action<AppConfig>({});

  return await queryWasmSmart(client, { contract: addresses.perps, msg, height });
}
