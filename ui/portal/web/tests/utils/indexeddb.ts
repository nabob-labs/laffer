import type { Page } from "@playwright/test";

/**
 * Get a value from IndexedDB storage used by Zustand persist
 * The app uses "laffer" as the database name and "velox" as the store name
 * Keys are prefixed with "velox." (e.g., "app.settings" -> "velox.app.settings")
 * Data is stored as JSON string in format: { state: { value: T, ... }, version: number }
 */
export async function getIndexedDBValue<T>(page: Page, key: string): Promise<T | null> {
  // Add the "velox." prefix that createAsyncStorage uses
  const prefixedKey = `velox.${key}`;

  return await page.evaluate(async (storageKey) => {
    return new Promise((resolve) => {
      const request = indexedDB.open("laffer");

      request.onsuccess = () => {
        const db = request.result;
        try {
          const tx = db.transaction("velox", "readonly");
          const store = tx.objectStore("velox");
          const getRequest = store.get(storageKey);

          getRequest.onsuccess = () => {
            const result = getRequest.result;
            if (!result) {
              resolve(null);
              return;
            }

            // Data might be stored as JSON string, parse it
            let parsed = result;
            if (typeof result === "string") {
              try {
                parsed = JSON.parse(result);
              } catch {
                resolve(null);
                return;
              }
            }

            // Zustand persist stores: { state: { value: T, ... }, version: number }
            resolve(parsed?.state?.value ?? null);
          };

          getRequest.onerror = () => resolve(null);
        } catch {
          resolve(null);
        }
      };

      request.onerror = () => resolve(null);
    });
  }, prefixedKey);
}

/**
 * Verify theme is applied to document
 */
export async function getAppliedTheme(page: Page): Promise<string> {
  return await page.evaluate(() => {
    const root = document.documentElement;
    if (root.classList.contains("dark")) return "dark";
    if (root.classList.contains("light")) return "light";
    return "system";
  });
}

/**
 * Get stored favorites from IndexedDB
 */
export async function getStoredFavorites(page: Page): Promise<string[]> {
  return (await getIndexedDBValue<string[]>(page, "app.applets")) ?? [];
}

/**
 * Get stored settings from IndexedDB
 */
export async function getStoredSettings(
  page: Page,
): Promise<Record<string, unknown> | null> {
  return await getIndexedDBValue(page, "app.settings");
}

/**
 * Wait for IndexedDB to be ready and hydrated
 */
export async function waitForStorageHydration(page: Page, timeout = 5000): Promise<void> {
  await page.waitForFunction(
    () => {
      // Check if page is fully loaded
      return document.readyState === "complete";
    },
    { timeout },
  );
  // Wait for IndexedDB operations to complete
  await page.waitForTimeout(500);
}

/**
 * Debug helper: List all keys in the IndexedDB store
 */
export async function listIndexedDBKeys(page: Page): Promise<string[]> {
  return await page.evaluate(async () => {
    return new Promise((resolve) => {
      const request = indexedDB.open("laffer");

      request.onsuccess = () => {
        const db = request.result;
        try {
          const tx = db.transaction("velox", "readonly");
          const store = tx.objectStore("velox");
          const getAllKeysRequest = store.getAllKeys();

          getAllKeysRequest.onsuccess = () => {
            resolve(getAllKeysRequest.result as string[]);
          };

          getAllKeysRequest.onerror = () => resolve([]);
        } catch {
          resolve([]);
        }
      };

      request.onerror = () => resolve([]);
    });
  });
}

/**
 * Debug helper: Get raw value from IndexedDB without parsing
 */
export async function getRawIndexedDBValue(page: Page, key: string): Promise<unknown> {
  const prefixedKey = `velox.${key}`;

  return await page.evaluate(async (storageKey) => {
    return new Promise((resolve) => {
      const request = indexedDB.open("laffer");

      request.onsuccess = () => {
        const db = request.result;
        try {
          const tx = db.transaction("velox", "readonly");
          const store = tx.objectStore("velox");
          const getRequest = store.get(storageKey);

          getRequest.onsuccess = () => {
            resolve(getRequest.result);
          };

          getRequest.onerror = () => resolve(null);
        } catch {
          resolve(null);
        }
      };

      request.onerror = () => resolve(null);
    });
  }, prefixedKey);
}
