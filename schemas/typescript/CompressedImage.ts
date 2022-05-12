// Generated from CompressedImage by @foxglove/message-schemas

import { Time } from "./Time";

/** A raw image */
export type CompressedImage = {
  /** Timestamp of image */
  timestamp: Time;

  /** Compressed image data */
  data: Uint8Array;

  /** Image format */
  format: string;
};
