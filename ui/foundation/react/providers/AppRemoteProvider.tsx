import { createContext } from "../utils/context";
import { requestRemote, useConfig, type WindowVeloxStore } from "@laffer/store";

import type { PropsWithChildren } from "react";
import type React from "react";
import type { AppState } from "./AppProvider";
import type { ToastController } from "../types/toast";

export interface WindowVeloxRemoteApp extends WindowVeloxStore {
  velox: WindowVeloxStore["velox"] & {
    settings: AppState["settings"];
  };
}

declare let window: WindowVeloxRemoteApp;

const navigate = (to: string, options?: { replace?: boolean }) => {
  requestRemote("navigate", to, options);
};

const hideModal = () => {
  requestRemote("hideModal");
};

const showModal = (modalName: string, props?: Record<string, unknown>) => {
  requestRemote("showModal", { modalName, props });
};

const toast = {
  success: (toastMsg, options) => {
    requestRemote<string>("toast", "success", toastMsg, options);
  },
  error: (toastMsg, options) => {
    requestRemote<string>("toast", "error", toastMsg, options);
  },
} as ToastController;

const [RemoteContextProvider, useRemoteApp] = createContext<AppState>();

export { useRemoteApp };

export const AppRemoteProvider: React.FC<PropsWithChildren> = ({ children }) => {
  const { subscriptions } = useConfig();

  return (
    <RemoteContextProvider
      value={
        {
          subscriptions,
          toast,
          settings: window.velox.settings,
          navigate,
          showModal,
          hideModal,
        } as AppState
      }
    >
      {children}
    </RemoteContextProvider>
  );
};
