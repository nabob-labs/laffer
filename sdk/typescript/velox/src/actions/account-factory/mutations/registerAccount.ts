import { getAppConfig } from "@laffer/sdk";
import { type ExecuteReturnType, execute } from "../../app/mutations/execute.js";

import { getAction } from "@laffer/sdk/actions";
import type { Address, Funds, Transport, TxParameters } from "@laffer/sdk/types";
import type { AppConfig, VeloxClient, Signer, TypedDataParameter } from "../../../types/index.js";

export type RegisterAccountParameters = {
  sender: Address;
  funds?: Funds;
};

export type RegisterAccountReturnType = ExecuteReturnType;

export async function registerAccount<transport extends Transport>(
  client: VeloxClient<transport, Signer>,
  parameters: RegisterAccountParameters,
  txParameters: TxParameters = {},
): RegisterAccountReturnType {
  const { sender, funds = {} } = parameters;
  const { gasLimit } = txParameters;
  const msg = { registerAccount: {} };

  const getAppConfigAction = getAction(client, getAppConfig, "getAppConfig");

  const { addresses } = await getAppConfigAction<AppConfig>({});

  const typedData: TypedDataParameter = {
    type: [{ name: "register_account", type: "RegisterAccount" }],
    extraTypes: {
      RegisterAccount: [],
    },
  };

  return await execute(client, {
    execute: {
      contract: addresses.accountFactory,
      msg,
      typedData,
      funds,
    },
    sender,
    gasLimit,
  });
}
