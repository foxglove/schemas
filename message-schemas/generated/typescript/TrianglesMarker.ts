// Generated from TrianglesMarker by @foxglove/message-schemas

import { Color } from "./Color";
import { Duration } from "./Duration";
import { KeyValuePair } from "./KeyValuePair";
import { Point3 } from "./Point3";
import { Pose } from "./Pose";
import { Time } from "./Time";
import { TrianglesType } from "./TrianglesType";

/** A marker representing a set of triangles or a surface tiled by triangles */
export type TrianglesMarker = {
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

  /** Drawing primitive to use for triangles */
  type: TrianglesType;

  /** Origin of triangles relative to reference frame */
  pose: Pose;

  /** Vertices to use for triangles */
  points: Point3[];

  /** Solid color to use for the whole shape. One of `color` or `colors` must be provided. */
  color: Color;

  /** Per-vertex colors (if specified, must have the same length as `points`). One of `color` or `colors` should be provided. */
  colors: Color[];

  /** Indexes into the `points` and `colors` attribute arrays, which can be used to avoid duplicating attribute data. */
  indices: number[];
};
