// Generated from CompressedImage by @foxglove/schemas

import { Time } from "./Time";

/** A compressed image */
export type CompressedImage = {
  /** Timestamp of image */
  timestamp: Time;

  /** Compressed image data */
  data: Uint8Array;

  /** Image format */
  format: string;
};
