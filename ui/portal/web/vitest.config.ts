import { defineConfig } from "vitest/config";
import path from "node:path";

const root = path.resolve(__dirname, "../../..");

export default defineConfig({
  test: {
    environment: "jsdom",
    include: ["tests/**/*.test.{ts,tsx}", "src/**/*.test.{ts,tsx}"],
    globals: true,
    setupFiles: ["tests/setup.ts"],
  },
  resolve: {
    alias: {
      "@laffer/velox/utils": path.resolve(root, "sdk/typescript/velox/src/utils/index.ts"),
      "@laffer/velox/types": path.resolve(root, "sdk/typescript/velox/src/types/index.ts"),
      "@laffer/velox": path.resolve(root, "sdk/typescript/velox/src/index.ts"),
      "@laffer/sdk/utils": path.resolve(root, "sdk/typescript/bolt/src/utils/index.ts"),
    },
  },
});
