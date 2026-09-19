import type { Address } from "@laffer/sdk/types";
import type { AccountIndex, User } from "../types/account.js";

export type ToAccountParameters = {
  user: User;
  accountIndex: AccountIndex;
  address: Address;
};

export function toAccount(parameters: ToAccountParameters) {
  const { user, accountIndex, address } = parameters;

  return {
    index: accountIndex,
    owner: user.index,
    address: address as Address,
  };
}
