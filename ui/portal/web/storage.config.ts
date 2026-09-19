import { get, set, del, createStore } from "idb-keyval";

import type { AbstractStorage } from "@laffer/store/types";

const store = createStore("laffer", "velox");

export function createIndexedDBStorage(): AbstractStorage {
  return {
    async getItem<T>(key: string): Promise<T | null> {
      const result = await get<T>(key, store);
      if (!result) return null;
      return result;
    },
    async setItem(key: string, data: string): Promise<void> {
      await set(key, data, store);
    },
    async removeItem(key: string): Promise<void> {
      await del(key, store);
    },
  };
}
