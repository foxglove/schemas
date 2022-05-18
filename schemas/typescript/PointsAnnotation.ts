// Generated from PointsAnnotation by @foxglove/schemas

import { Color } from "./Color";
import { Point2 } from "./Point2";
import { PointsAnnotationType } from "./PointsAnnotationType";
import { Time } from "./Time";

/** An array of points on a 2D image */
export type PointsAnnotation = {
  /** Timestamp of annotation */
  timestamp: Time;

  /** Type of points annotation to draw */
  type: PointsAnnotationType;

  /** Points in 2D image coordinates */
  points: Point2[];

  /** Outline colors */
  outline_colors: Color[];

  /** Fill color */
  fill_color: Color;
};
