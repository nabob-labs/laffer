import type { Json } from "@laffer/sdk/types";

export type DataChannelConfig = {
  rtcConfiguration: RTCConfiguration;
  channelName: string;
  logs: boolean;
};

export type DataChannelMessage = {
  id: string;
  type: string;
  message: Json;
};
