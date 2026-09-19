import { useConnectors } from "@laffer/store";
import { Button, type ButtonProps } from "./Button";
import { Modals, useApp } from "@laffer/foundation";
import { withResolvers } from "@laffer/velox/utils";

import { m } from "@laffer/foundation/paraglide/messages.js";

import type React from "react";
import type { EIP1193Provider } from "@laffer/store/types";
import { useMutation } from "@tanstack/react-query";

interface ConnectWalletWithModalProps extends Omit<ButtonProps, "onClick" | "children"> {
  onWalletSelected: (walletId: string) => void;
}

export const ConnectWalletWithModal: React.FC<ConnectWalletWithModalProps> = ({
  onWalletSelected,
  ...buttonProps
}) => {
  const { showModal } = useApp();
  const connectors = useConnectors();

  const { isPending, mutateAsync } = useMutation({
    mutationFn: async () => {
      const { promise, resolve: onWalletSelect, reject: onReject } = withResolvers<string>();

      showModal(Modals.WalletSelector, {
        onWalletSelect,
        onReject,
      });

      const walletId = await promise;
      const connector = connectors.find((c) => c.id === walletId);
      if (!connector) return onReject();

      try {
        const provider = await (
          connector as unknown as { getProvider: () => Promise<EIP1193Provider> }
        ).getProvider();
        await provider.request({ method: "eth_requestAccounts" });
        onWalletSelected(walletId);
      } catch {
        onReject();
      }
    },
  });

  return (
    <Button {...buttonProps} onClick={() => mutateAsync()} isLoading={isPending}>
      {m["signin.connectWallet"]()}
    </Button>
  );
};
