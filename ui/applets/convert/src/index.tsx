import ReactDOM from "react-dom/client";
import { useState } from "react";
import { Convert } from "./applet";
import { VeloxRemoteProvider } from "@laffer/store";
import { AppRemoteProvider } from "@laffer/applets-kit";
import { QueryClientProvider, QueryClient } from "@tanstack/react-query";

import "@laffer/foundation/fonts/ABCDiatypeRounded/normal/800.css";
import "@laffer/foundation/fonts/ABCDiatypeRounded/normal/700.css";
import "@laffer/foundation/fonts/ABCDiatypeRounded/normal/500.css";
import "@laffer/foundation/fonts/ABCDiatypeRounded/normal/400.css";

import "@laffer/foundation/fonts/ABCDiatypeRounded/mono/600.css";
import "@laffer/foundation/fonts/ABCDiatypeRounded/mono/500.css";

import "@laffer/foundation/fonts/Exposure/italic/400.css";
import "@laffer/foundation/fonts/Exposure/italic/700.css";
import "./global.css";

import type React from "react";

const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      refetchOnWindowFocus: false,
      retry: 0,
    },
  },
});

export const ConvertApplet: React.FC = () => {
  const [{ from, to }, onChangePair] = useState({ from: "USDC", to: "ETH" });

  return (
    <div className="w-full md:max-w-[25rem] mx-auto flex flex-col p-4 pt-6 gap-4 min-h-[100svh] md:min-h-fit text-ink-primary-900">
      <Convert pair={{ from, to }} onChangePair={onChangePair}>
        <Convert.Header />
        <Convert.Form />
        <Convert.Details />
        <Convert.Trigger />
      </Convert>
    </div>
  );
};

const container = document.getElementById("root");
if (!container) throw new Error("No root element found");

const root = ReactDOM.createRoot(container);
root.render(
  <QueryClientProvider client={queryClient}>
    <VeloxRemoteProvider>
      <AppRemoteProvider>
        <ConvertApplet />
      </AppRemoteProvider>
    </VeloxRemoteProvider>
  </QueryClientProvider>,
);
