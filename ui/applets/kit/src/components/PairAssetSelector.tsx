import { useControlledState } from "@laffer/foundation";
import { useAppConfig, useConfig } from "@laffer/store";

import { CoinSelector } from "./CoinSelector";
import { Skeleton } from "./Skeleton";

import type React from "react";

type PairAssetSelectorProps = {
  value: string;
  onChange: (denom: string) => void;
};

export const PairAssetSelector: React.FC<PairAssetSelectorProps> = ({ value, onChange }) => {
  const { coins } = useConfig();
  const { data: config } = useAppConfig();

  const pairs = Object.fromEntries(
    Object.entries(config.pairs).filter(([_, v]) => v.params.curveInvariant !== "xyk"),
  );

  const pairCoins = Object.keys(pairs);

  const coinPairs = Object.values(coins.byDenom).filter((c) => pairCoins.includes(c.denom));

  const [state, setState] = useControlledState<string>(value, onChange);

  return coinPairs.length ? (
    <CoinSelector
      coins={Object.values(coins.byDenom).filter(
        (c) => pairCoins.includes(c.denom) && c.denom !== "velox",
      )}
      value={state}
      onChange={(v) => setState(coins.byDenom[v].symbol)}
    />
  ) : (
    <Skeleton className="w-36 h-11" />
  );
};
