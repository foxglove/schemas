// Generated from CameraCalibration by @foxglove/message-schemas

import { Time } from "./Time";

/** Camera calibration parameters */
export type CameraCalibration = {
  /** Timestamp of calibration data */
  timestamp: Time;

  /** Image width */
  width: number;

  /** Image height */
  height: number;

  /** Name of distortion model */
  distortion_model: string;

  /** Distortion parameters */
  D: number[];

  /** Intrinsic camera matrix (3x3 row-major matrix) */
  K: [number, number, number, number, number, number, number, number, number];

  /** Rectification matrix (3x3 row-major matrix) */
  R: [number, number, number, number, number, number, number, number, number];

  /** Projection/camera matrix (3x4 row-major matrix) */
  P: [number, number, number, number, number, number, number, number, number, number, number, number];
};
