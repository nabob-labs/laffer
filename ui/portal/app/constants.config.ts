import { m } from "@laffer/foundation/paraglide/messages.js";

import type { AppletMetadata } from "@laffer/store/types";

export const WS_URI = "wss://webrtc.velox.exchange";

export const DEFAULT_SESSION_EXPIRATION = 24 * 60 * 60 * 1000; // 24 hours

export const PRIVY_ERRORS_MAPPING = {
  "User already has one email account linked": m["auth.errors.userNotFound"](),
  authFailed: m["auth.errors.authFailed"](),
  "User does not exist": m["auth.errors.userNotFound"](),
};

const translations = m as unknown as Record<string, () => string>;
export const APPLETS: Record<string, AppletMetadata> = Object.keys(translations)
  .filter((k) => /^applets\..*\.id$/.test(k))
  .reduce((acc, key) => {
    const [_, id] = key.split(".");
    acc[id] = {
      id,
      title: translations[`applets.${id}.title`](),
      description: translations[`applets.${id}.description`](),
      img: translations[`applets.${id}.img`](),
      keywords: translations[`applets.${id}.keywords`]().split(","),
      path: translations[`applets.${id}.path`](),
    };
    return acc;
  }, Object.create({}));

export const ASSETS = {
  trade: require("@laffer/foundation/images/emojis/simple/protrading.svg"),
  convert: require("@laffer/foundation/images/emojis/simple/swap.svg"),
  bridge: require("@laffer/foundation/images/emojis/simple/moneybag.svg"),
  earn: require("@laffer/foundation/images/emojis/simple/pig.svg"),
  transfer: require("@laffer/foundation/images/emojis/simple/money.svg"),
  "create-account": require("@laffer/foundation/images/emojis/simple/wizard.svg"),
  settings: require("@laffer/foundation/images/emojis/simple/settings.svg"),
  devtool: require("@laffer/foundation/images/emojis/simple/factory-2.svg"),
};

export const COINS = {
  SOL: require("@laffer/foundation/images/coins/sol.svg"),
  USDC: require("@laffer/foundation/images/coins/usdc.svg"),
  ETH: require("@laffer/foundation/images/coins/eth.svg"),
  XRP: require("@laffer/foundation/images/coins/xrp.svg"),
  BTC: require("@laffer/foundation/images/coins/bitcoin.svg"),
};
