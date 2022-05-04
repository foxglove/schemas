// Generated from CylinderMarker by @foxglove/message-schemas

import { Color } from "./Color";
import { Duration } from "./Duration";
import { KeyValuePair } from "./KeyValuePair";
import { Pose } from "./Pose";
import { Time } from "./Time";

/** A marker representing a cylinder or elliptic cylinder */
export type CylinderMarker = {
  /** Timestamp of the marker */
  timestamp: Time;

  /** Frame of reference */
  frame_id: string;

  /** Namespace into which the marker should be grouped. A marker will replace any prior marker on the same topic with the same `namespace` and `id`. */
  namespace: string;

  /** Identifier for the marker. A marker will replace any prior marker on the same topic with the same `namespace` and `id`. */
  id: string;

  /** Length of time (relative to `timestamp`) after which the marker should be automatically removed. Zero value indicates the marker should remain visible until it is replaced or deleted. */
  lifetime: Duration;

  /** Whether the marker should keep its location in the fixed frame (false) or follow the frame specified in `frame_id` as it moves relative to the fixed frame (true) */
  frame_locked: boolean;

  /** Additional user-provided metadata associated with the marker. Keys must be unique. */
  metadata: KeyValuePair[];

  /** Position of the center of the cylinder and orientation of the cylinder. The cylinder's flat faces are perpendicular to the z-axis. */
  pose: Pose;

  /** Radius of the cylinder at min z */
  bottom_radius: number;

  /** Radius of the cylinder at max z */
  top_radius: number;

  /** Height of the cylinder along the z axis */
  height: number;

  /** Color of the sphere */
  color: Color;
};
