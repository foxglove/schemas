// Generated by https://github.com/foxglove/schemas

import { Color } from "./Color";
import { LineType } from "./LineType";
import { Point3 } from "./Point3";
import { Pose } from "./Pose";

/** A primitive representing a series of points connected by lines */
export type LinePrimitive = {
  /** Drawing primitive to use for lines */
  type: LineType;

  /** Origin of lines relative to reference frame */
  pose: Pose;

  /** Line thickness */
  thickness: number;

  /** Indicates whether `thickness` is a fixed size in screen pixels (true), or specified in world coordinates and scales with distance from the camera (false) */
  scale_invariant: boolean;

  /** Points along the line */
  points: Point3[];

  /** Solid color to use for the whole line. One of `color` or `colors` must be provided. */
  color: Color;

  /** Per-point colors (if specified, must have the same length as `points`). One of `color` or `colors` must be provided. */
  colors: Color[];

  /**
   * Indices into the `points` and `colors` attribute arrays, which can be used to avoid duplicating attribute data.
   * 
   * If omitted or empty, indexing will not be used. This default behavior is equivalent to specifying [0, 1, ..., N-1] for the indices (where N is the number of `points` provided).
   */
  indices: number[];
};