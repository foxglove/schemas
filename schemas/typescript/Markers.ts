// Generated from Markers by @foxglove/message-schemas

import { ArrowMarker } from "./ArrowMarker";
import { CubeMarker } from "./CubeMarker";
import { CylinderMarker } from "./CylinderMarker";
import { LineMarker } from "./LineMarker";
import { MarkerDeletion } from "./MarkerDeletion";
import { ModelMarker } from "./ModelMarker";
import { SphereMarker } from "./SphereMarker";
import { TextMarker } from "./TextMarker";
import { TrianglesMarker } from "./TrianglesMarker";

/** A list that can contain any number of any type of marker */
export type Markers = {
  /** Marker deletion actions */
  deletions: MarkerDeletion[];

  /** Arrow markers */
  arrows: ArrowMarker[];

  /** Cube markers */
  cubes: CubeMarker[];

  /** Sphere markers */
  spheres: SphereMarker[];

  /** Cylinder markers */
  cylinders: CylinderMarker[];

  /** Line markers */
  lines: LineMarker[];

  /** Triangles markers */
  triangles: TrianglesMarker[];

  /** Text markers */
  texts: TextMarker[];

  /** Model markers */
  models: ModelMarker[];
};
