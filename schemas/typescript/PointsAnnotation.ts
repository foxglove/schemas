// Generated from PointsAnnotation by @foxglove/message-schemas

import { Color } from "./Color";
import { Point2 } from "./Point2";
import { PointsAnnotationType } from "./PointsAnnotationType";
import { Time } from "./Time";

/** An array of points to be superimposed onto 2D images */
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
