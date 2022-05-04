// Generated from TriangleType by @foxglove/message-schemas

/** An enumeration indicating how input points should be interpreted to create triangles */
export enum TriangleType {
  /** 0-1-2, 3-4-5, ... */
  TRIANGLE_LIST = 0,

  /** 0-1-2, 1-2-3, 2-3-4, ... */
  TRIANGLE_STRIP = 1,

  /** 0-1-2, 0-2-3, 0-3-4, ... */
  TRIANGLE_FAN = 2,
}
