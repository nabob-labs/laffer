import { defineChain } from "../defineChain.js";

export const local = /*#__PURE__*/ defineChain({
  id: "localvelox-1",
  name: "Local",
  nativeCoin: "velox",
  blockExplorer: {
    name: "Local Explorer",
    txPage: "/tx/${txHash}",
    accountPage: "/account/${address}",
    contractPage: "/contract/${address}",
  },
  urls: {
    indexer: "http://localhost:8080",
  },
});
