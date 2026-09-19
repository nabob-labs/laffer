import { queryWasmSmart } from "@laffer/sdk";
import type { Client, Prettify, Transport } from "@laffer/sdk/types";

import { getAction, getAppConfig } from "@laffer/sdk/actions";
import type { Chain, Signer } from "@laffer/sdk/types";
import type { AppConfig } from "../../../types/app.js";
import type { PerpsQueryMsg, VaultSnapshot } from "../../../types/perps.js";

export type GetVaultSnapshotsParameters = Prettify<{
  min?: number;
  max?: number;
  height?: number;
}>;

export type GetVaultSnapshotsReturnType = Promise<Record<string, VaultSnapshot>>;

export async function getVaultSnapshots<
  chain extends Chain | undefined,
  signer extends Signer | undefined,
>(
  client: Client<Transport, chain, signer>,
  parameters?: GetVaultSnapshotsParameters,
): GetVaultSnapshotsReturnType {
  const { min, max, height = 0 } = parameters ?? {};

  const action = getAction(client, getAppConfig, "getAppConfig");
  const { addresses } = await action<AppConfig>({});

  const msg: PerpsQueryMsg = {
    vaultSnapshots: {
      min: min != null ? String(min) : undefined,
      max: max != null ? String(max) : undefined,
    },
  };

  return queryWasmSmart(client, {
    contract: addresses.perps,
    msg,
    height,
  });
}
