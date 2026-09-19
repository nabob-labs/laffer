"use client";

import { uid } from "@laffer/velox/utils";
import { deserializeJson, serializeJson } from "@laffer/velox/encoding";

import type { Chain } from "@laffer/velox/types";
import type { NativeCoin } from "./types/coin.js";
import type { RemoteResponse } from "./types/remote.js";
import type { Connection } from "./types/connector.js";

export interface WindowVeloxStore extends Window {
  velox: {
    chain: Chain;
    coins: Record<string, NativeCoin>;
    connection?: Omit<Connection, "connector">;
  };
  ReactNativeWebView: {
    postMessage: (message: string) => void;
  };
}

declare let window: WindowVeloxStore;

export const requestRemote = async <T = unknown>(
  method: string,
  ...args: unknown[]
): Promise<T> => {
  const id = uid();

  const message = {
    id,
    method,
    args,
  };

  return await new Promise((resolve, reject) => {
    const timeoutId = setTimeout(() => {
      window.removeEventListener("message", receiveResponse);
      reject(new Error("Request timed out"));
    }, 30_000);

    const receiveResponse = (e: MessageEvent) => {
      const message = deserializeJson<RemoteResponse<T>>(e.data);

      if (!message || message.type !== "velox-remote") {
        return;
      }

      clearTimeout(timeoutId);

      if (message.id !== id) return;

      window.removeEventListener("message", receiveResponse);

      const { data, error } = message;

      if (error) reject(error);

      resolve(data as T);
    };

    window.addEventListener("message", receiveResponse);

    window.ReactNativeWebView?.postMessage(serializeJson(message));
  });
};
