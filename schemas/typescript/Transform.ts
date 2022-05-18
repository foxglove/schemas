// Generated from Transform by @foxglove/schemas

import { Quaternion } from "./Quaternion";
import { Time } from "./Time";
import { Vector3 } from "./Vector3";

/** A transform in 3D space */
export type Transform = {
  /** Transform time */
  timestamp: Time;

  /** Translation component of the transform */
  translation: Vector3;

  /** Rotation component of the transform */
  rotation: Quaternion;
};
