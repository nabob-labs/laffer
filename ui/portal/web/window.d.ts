import type { Chain } from "@laffer/velox/types";

declare global {
  interface Window {
    velox: {
      chain: Chain;
      urls: {
        faucetUrl: string;
        questUrl: string;
        upUrl: string;
        pointsUrl: string;
      };
      banner?: string;
      enabledFeatures?: string[];
    };
  }
}
