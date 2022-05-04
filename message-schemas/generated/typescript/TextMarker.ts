// Generated from TextMarker by @foxglove/message-schemas

import { Color } from "./Color";
import { Duration } from "./Duration";
import { KeyValuePair } from "./KeyValuePair";
import { Pose } from "./Pose";
import { Time } from "./Time";

/** A marker representing a text label */
export type TextMarker = {
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

  /** Position of the center of the text box and orientation of the text. Identity orientation means the text is oriented in the xy-plane and flows from -x to +x. */
  pose: Pose;

  /** Whether the text should respect `pose.orientation` (false) or always face the camera (true) */
  billboard: boolean;

  /** Font size (height of one line of text) */
  font_size: number;

  /** Indicates whether `font_size` is a fixed size in screen pixels (true), or specified in world coordinates and scales with distance from the camera (false) */
  scale_invariant: boolean;

  /** Color of the text */
  color: Color;

  /** Text */
  text: string;
};
