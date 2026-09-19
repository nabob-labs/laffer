import { Modals, useApp } from "@laffer/foundation";
import { useAccount } from "@laffer/store";
import { cloneElement, type PropsWithChildren, type ReactElement } from "react";
import { m } from "@laffer/foundation/paraglide/messages.js";

import type React from "react";

export const AuthenticatedButton: React.FC<PropsWithChildren> = ({ children }) => {
  const { showModal } = useApp();
  const { isConnected } = useAccount();
  if (isConnected) return children;

  const Button = cloneElement(
    children as ReactElement<{ type?: string; onClick?: () => void }>,
    {
      type: "button",
      onClick: () => showModal(Modals.Authenticate),
    },
    m["common.signin"](),
  );

  return Button;
};
