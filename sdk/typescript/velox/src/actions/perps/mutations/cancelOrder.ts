import { getAppConfig } from "@laffer/sdk";
import { getAction } from "@laffer/sdk/actions";
import { execute } from "../../app/mutations/execute.js";

import type { Address, Transport } from "@laffer/sdk/types";
import type { SignAndBroadcastTxReturnType } from "../../app/mutations/signAndBroadcastTx.js";
import type {
  AppConfig,
  VeloxClient,
  PerpsCancelOrderRequest,
  Signer,
  TypedDataParameter,
} from "../../../types/index.js";

export type CancelPerpsOrderParameters = {
  sender: Address;
  request: PerpsCancelOrderRequest;
};

export type CancelPerpsOrderReturnType = SignAndBroadcastTxReturnType;

export async function cancelPerpsOrder<transport extends Transport>(
  client: VeloxClient<transport, Signer>,
  parameters: CancelPerpsOrderParameters,
): CancelPerpsOrderReturnType {
  const { sender, request } = parameters;

  const getAppConfigAction = getAction(client, getAppConfig, "getAppConfig");
  const { addresses } = await getAppConfigAction<AppConfig>({});

  const msg = {
    trade: {
      cancelOrder: request,
    },
  };

  const cancelOrderTypedData = (() => {
    if (request === "all") {
      return { CancelOrder: [] };
    }
    if ("one" in request) {
      return { CancelOrder: [{ name: "one", type: "string" }] };
    }
    // `oneByClientOrderId` — the message is camelCase here; the
    // outgoing JSON is snake_cased by `composeTxTypedData` via
    // `recursiveTransform`, so the typed-data field name must be the
    // snake_case form the contract sees.
    return {
      CancelOrder: [{ name: "one_by_client_order_id", type: "string" }],
    };
  })();

  const typedData: TypedDataParameter = {
    type: [{ name: "trade", type: "Trade" }],
    extraTypes: {
      Trade: [{ name: "cancel_order", type: "CancelOrder" }],
      ...cancelOrderTypedData,
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
