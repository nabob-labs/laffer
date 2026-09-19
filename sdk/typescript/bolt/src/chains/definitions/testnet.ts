import { defineChain } from "../defineChain.js";

export const testnet = /*#__PURE__*/ defineChain({
  id: "velox-testnet-1",
  name: "Testnet",
  nativeCoin: "velox",
  blockExplorer: {
    name: "Testnet Explorer",
    txPage: "/tx/${txHash}",
    accountPage: "/account/${address}",
    contractPage: "/contract/${address}",
  },
  urls: {
    indexer: "https://api-testnet.velox.zone",
  },
});
