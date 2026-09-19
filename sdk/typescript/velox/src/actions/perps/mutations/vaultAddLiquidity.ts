import { getAppConfig } from "@laffer/sdk";
import { getAction } from "@laffer/sdk/actions";
import { execute } from "../../app/mutations/execute.js";

import type { Address, Transport } from "@laffer/sdk/types";
import type { SignAndBroadcastTxReturnType } from "../../app/mutations/signAndBroadcastTx.js";
import type { AppConfig, VeloxClient, Signer, TypedDataParameter } from "../../../types/index.js";

export type VaultAddLiquidityParameters = {
  sender: Address;
  amount: string;
  minSharesToMint?: string;
};

export type VaultAddLiquidityReturnType = SignAndBroadcastTxReturnType;

export async function vaultAddLiquidity<transport extends Transport>(
  client: VeloxClient<transport, Signer>,
  parameters: VaultAddLiquidityParameters,
): VaultAddLiquidityReturnType {
  const { sender, amount, minSharesToMint } = parameters;

  const getAppConfigAction = getAction(client, getAppConfig, "getAppConfig");

  const { addresses } = await getAppConfigAction<AppConfig>({});

  const msg = {
    vault: {
      addLiquidity: {
        amount,
        ...(minSharesToMint ? { minSharesToMint } : {}),
      },
    },
  };

  const typedData: TypedDataParameter = {
    type: [{ name: "vault", type: "Vault" }],
    extraTypes: {
      Vault: [{ name: "add_liquidity", type: "AddLiquidity" }],
      AddLiquidity: [
        { name: "amount", type: "string" },
        ...(minSharesToMint ? [{ name: "min_shares_to_mint", type: "string" }] : []),
      ],
    },
  };

  return await execute(client, {
    sender,
    execute: {
      msg,
      typedData,
      contract: addresses.perps,
    },
  });
}
