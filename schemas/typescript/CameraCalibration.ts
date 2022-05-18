// Generated from CameraCalibration by @foxglove/schemas

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

  /**
   * Intrinsic camera matrix (3x3 row-major matrix)
   * 
   * A 3x3 row-major matrix for the raw (distorted) image.
   * 
   * Projects 3D points in the camera coordinate frame to 2D pixel coordinates using the focal lengths (fx, fy) and principal point (cx, cy).
   * 
   * ```
   *     [fx  0 cx]
   * K = [ 0 fy cy]
   *     [ 0  0  1]
   * ```
   */
  K: [number, number, number, number, number, number, number, number, number];

  /**
   * Rectification matrix (3x3 row-major matrix)
   * 
   * A rotation matrix aligning the camera coordinate system to the ideal stereo image plane so that epipolar lines in both stereo images are parallel.
   */
  R: [number, number, number, number, number, number, number, number, number];

  /**
   * Projection/camera matrix (3x4 row-major matrix)
   * 
   * ```
   *     [fx'  0  cx' Tx]
   * P = [ 0  fy' cy' Ty]
   *     [ 0   0   1   0]
   * ```
   * 
   * By convention, this matrix specifies the intrinsic (camera) matrix of the processed (rectified) image. That is, the left 3x3 portion is the normal camera intrinsic matrix for the rectified image.
   * 
   * It projects 3D points in the camera coordinate frame to 2D pixel coordinates using the focal lengths (fx', fy') and principal point (cx', cy') - these may differ from the values in K.
   * 
   * For monocular cameras, Tx = Ty = 0. Normally, monocular cameras will also have R = the identity and P[1:3,1:3] = K.
   * 
   * For a stereo pair, the fourth column [Tx Ty 0]' is related to the position of the optical center of the second camera in the first camera's frame. We assume Tz = 0 so both cameras are in the same stereo image plane. The first camera always has Tx = Ty = 0. For the right (second) camera of a horizontal stereo pair, Ty = 0 and Tx = -fx' * B, where B is the baseline between the cameras.
   * 
   * Given a 3D point [X Y Z]', the projection (x, y) of the point onto the rectified image is given by:
   * 
   * ```
   * [u v w]' = P * [X Y Z 1]'
   *        x = u / w
   *        y = v / w
   * ```
   * 
   * This holds for both images of a stereo pair.
   */
  P: [number, number, number, number, number, number, number, number, number, number, number, number];
};
