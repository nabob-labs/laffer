import { getAppConfig } from "@laffer/sdk";
import { getAction } from "@laffer/sdk/actions";
import { execute } from "../../app/mutations/execute.js";

import type { Address, Coins, Denom, Transport } from "@laffer/sdk/types";
import type { BroadcastTxSyncReturnType } from "../../app/mutations/broadcastTxSync.js";
import type {
  AppConfig,
  VeloxClient,
  DexExecuteMsg,
  Signer,
  TypedDataParameter,
} from "../../../types/index.js";

export type ProvideLiquidityParameters = {
  sender: Address;
  baseDenom: Denom;
  quoteDenom: Denom;
  funds: Coins;
};

export type ProvideLiquidityReturnType = BroadcastTxSyncReturnType;

export async function provideLiquidity<transport extends Transport>(
  client: VeloxClient<transport, Signer>,
  parameters: ProvideLiquidityParameters,
): ProvideLiquidityReturnType {
  const { baseDenom, quoteDenom, funds, sender } = parameters;

  const getAppConfigAction = getAction(client, getAppConfig, "getAppConfig");

  const { addresses } = await getAppConfigAction<AppConfig>({});

  const msg: DexExecuteMsg = {
    provideLiquidity: {
      baseDenom,
      quoteDenom,
    },
  };

  const typedData: TypedDataParameter = {
    type: [{ name: "provide_liquidity", type: "ProvideLiquidity" }],
    extraTypes: {
      ProvideLiquidity: [
        { name: "base_denom", type: "string" },
        { name: "quote_denom", type: "string" },
      ],
    },
  };

  return await execute(client, {
    sender,
    execute: {
      msg,
      typedData,
      contract: addresses.dex,
      funds,
    },
  });
}
