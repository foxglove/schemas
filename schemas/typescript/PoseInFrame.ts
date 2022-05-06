// Generated from PoseInFrame by @foxglove/message-schemas

import { Pose } from "./Pose";
import { Time } from "./Time";

/** A timestamped pose in a named coordinate frame */
export type PoseInFrame = {
  /** Timestamp of pose */
  timestamp: Time;

  /** Frame of reference for pose position and orientation */
  frame_id: string;

  /** Pose in 3D space */
  pose: Pose;
};
