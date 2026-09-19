import type { Result } from "@laffer/velox/types";

export type RemoteRequest<Args = any[]> = {
  id: string;
  type: "velox-remote";
  method: string;
  args: Args;
};

export type RemoteResponse<T> = Result<T> & {
  id: string;
  type: "velox-remote";
};
