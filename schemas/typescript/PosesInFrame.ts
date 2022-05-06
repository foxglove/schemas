// Generated from PosesInFrame by @foxglove/message-schemas

import { Pose } from "./Pose";
import { Time } from "./Time";

/** An array of timestamped poses in a named coordinate frame */
export type PosesInFrame = {
  /** Timestamp of pose */
  timestamp: Time;

  /** Frame of reference for pose position and orientation */
  frame_id: string;

  /** Poses in 3D space */
  poses: Pose[];
};
