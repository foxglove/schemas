// Generated from RawImage by @foxglove/schemas

import { Time } from "./Time";

/** A raw image */
export type RawImage = {
  /** Timestamp of image */
  timestamp: Time;

  /** Image width */
  width: number;

  /** Image height */
  height: number;

  /** Encoding of the raw image data (8UC1, 8UC3, 16UC1, 32FC1, bayer_bggr8, bayer_gbrg8, bayer_grbg8, bayer_rggb8, bgr8, mono8, mono16, rgb8, or yuv422) */
  encoding: string;

  /** Byte length of a single row */
  step: number;

  /** Raw image data */
  data: Uint8Array;
};
