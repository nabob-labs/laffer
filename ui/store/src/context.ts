"use client";

import { type PropsWithChildren, createContext, createElement } from "react";
import { Hydrate } from "./hydrate.js";
import { createConfig } from "./createConfig.js";
import { graphql } from "@laffer/velox";
import { remote } from "./connectors/remote.js";

import { ConnectionStatus, type Config, type State } from "./types/store.js";
import type { WindowVeloxStore } from "./remote.js";

declare let window: WindowVeloxStore;

export const VeloxStoreContext = createContext<Config | undefined>(undefined);

export type VeloxStoreProviderProps = {
  config: Config;
  initialState?: State;
  reconnectOnMount?: boolean;
};

export const VeloxStoreProvider: React.FC<React.PropsWithChildren<VeloxStoreProviderProps>> = (
  parameters: PropsWithChildren<VeloxStoreProviderProps>,
) => {
  const { children, config } = parameters;

  return createElement(
    Hydrate,
    parameters,
    createElement(VeloxStoreContext.Provider, { value: config }, children),
  );
};

export const VeloxRemoteProvider: React.FC<React.PropsWithChildren> = (parameters) => {
  const { children } = parameters;

  const chain = window.velox.chain;
  const connection = window.velox.connection;

  const config = createConfig({
    chain,
    transport: graphql(chain.urls.indexer, { batch: true }),
    coins: window.velox.coins,
    ssr: false,
    connectors: [remote()],
  });

  const connector = config.connectors.at(0)!;

  const initialState = connection
    ? {
        chainId: chain.id,
        isMipdLoaded: true,
        current: connector.uid,
        user: {
          index: connection.account.owner,
          username: `User #${connection.account.owner}`,
          status: "active" as const,
        },
        connectors: new Map([[connector.uid, { ...connection, connector }]]),
        status: ConnectionStatus.Connected,
      }
    : {
        chainId: chain.id,
        isMipdLoaded: true,
        current: null,
        user: undefined,
        connectors: new Map(),
        status: ConnectionStatus.Disconnected,
      };

  return createElement(
    Hydrate,
    { config, initialState, reconnectOnMount: false },
    createElement(VeloxStoreContext.Provider, { value: config }, children),
  );
};
