import { RouterProvider, createRouter } from "@tanstack/react-router";

import { useAccount, useConfig, usePublicClient } from "@laffer/store";

import { Spinner, useTheme, type UseThemeReturnType } from "@laffer/applets-kit";

import { routeTree } from "./app.pages";
import { ErrorPage } from "./components/foundation/ErrorPage";
import { queryClient } from "./queryClient";

import type { QueryClient } from "@tanstack/react-query";
import type {
  UseAccountReturnType,
  UseConfigReturnType,
  UsePublicClientReturnType,
} from "@laffer/store";

export const router = createRouter({
  routeTree,
  defaultPreload: "intent",
  defaultStaleTime: 5000,
  scrollRestoration: true,
  context: {} as RouterContext,
  defaultPendingComponent: () => (
    <div className="flex-1 w-full flex justify-center items-center h-screen">
      <Spinner size="lg" color="pink" />
    </div>
  ),
  defaultErrorComponent: ErrorPage,
});

declare module "@tanstack/react-router" {
  interface Register {
    router: typeof router;
  }
}

export interface RouterContext {
  client: UsePublicClientReturnType;
  account: UseAccountReturnType;
  config: UseConfigReturnType;
  theme: UseThemeReturnType;
  queryClient: QueryClient;
}

export const AppRouter: React.FC = () => {
  const account = useAccount();
  const config = useConfig();
  const client = usePublicClient();
  const theme = useTheme();

  return <RouterProvider router={router} context={{ account, config, client, theme, queryClient }} />;
};
