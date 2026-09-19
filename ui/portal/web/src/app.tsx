import { VeloxStoreProvider } from "@laffer/store";
import { QueryClientProvider } from "@tanstack/react-query";
import { config } from "~/store";

import { AppRouter, router } from "./app.router";
import { AppProvider } from "@laffer/foundation";
import { Toaster, toast } from "@laffer/applets-kit";
import { RootModal } from "./components/modals/RootModal";
import { StatusBadge } from "./components/foundation/StatusBadge";
import { queryClient } from "./queryClient";

import type React from "react";

import "../public/global.css";
import "@laffer/foundation/fonts/ABCDiatypeRounded/normal/800.css";
import "@laffer/foundation/fonts/ABCDiatypeRounded/normal/700.css";
import "@laffer/foundation/fonts/ABCDiatypeRounded/normal/500.css";
import "@laffer/foundation/fonts/ABCDiatypeRounded/normal/400.css";

import "@laffer/foundation/fonts/ABCDiatypeRounded/mono/600.css";
import "@laffer/foundation/fonts/ABCDiatypeRounded/mono/500.css";

import "@laffer/foundation/fonts/Exposure/italic/400.css";
import "@laffer/foundation/fonts/Exposure/italic/700.css";

export const App: React.FC = () => {
  return (
    <QueryClientProvider client={queryClient}>
      <VeloxStoreProvider config={config}>
        <AppProvider toast={toast} navigate={(to, options) => router.navigate({ to, ...options })}>
          <AppRouter />
          <RootModal />
          <Toaster />
        </AppProvider>
      </VeloxStoreProvider>
    </QueryClientProvider>
  );
};
