import { getAppConfig } from "@laffer/sdk";
import { getAction } from "@laffer/sdk/actions";
import { execute } from "../../app/mutations/execute.js";

import type { Address, Transport } from "@laffer/sdk/types";
import type { SignAndBroadcastTxReturnType } from "../../app/mutations/signAndBroadcastTx.js";
import type { AppConfig, VeloxClient, Signer, TypedDataParameter } from "../../../types/index.js";

export type SetReferralParameters = {
  sender: Address;
  referrer: number;
  referee: number;
};

export type SetReferralReturnType = SignAndBroadcastTxReturnType;

export async function setReferral<transport extends Transport>(
  client: VeloxClient<transport, Signer>,
  parameters: SetReferralParameters,
): SetReferralReturnType {
  const { sender, referrer, referee } = parameters;

  const getAppConfigAction = getAction(client, getAppConfig, "getAppConfig");
  const { addresses } = await getAppConfigAction<AppConfig>({});

  const msg = {
    referral: {
      setReferral: {
        referrer,
        referee,
      },
    },
  };

  const typedData: TypedDataParameter = {
    type: [{ name: "referral", type: "Referral" }],
    extraTypes: {
      Referral: [{ name: "set_referral", type: "SetReferral" }],
      SetReferral: [
        { name: "referrer", type: "uint32" },
        { name: "referee", type: "uint32" },
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
