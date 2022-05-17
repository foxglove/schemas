// Generated from PackedElementField by @foxglove/message-schemas

import { NumericType } from "./NumericType";

/** Defines a field within packed data. Commonly used to represent a field of a single element within a byte array containing multiple packed elements. */
export type PackedElementField = {
  /** Name of the field */
  name: string;

  /** Byte offset from start of data buffer */
  offset: number;

  /** Type of data in the field. Integers are stored using little-endian byte order. */
  type: NumericType;
};
