// Generated from DataField by @foxglove/message-schemas

import { NumericType } from "./NumericType";

/** List of fields for every element in a byte array */
export type DataField = {
  /** Name of the field */
  name: string;

  /** Byte offset from start of data buffer */
  offset: number;

  /** Type of data in the field. Integers are stored using little-endian byte order. */
  type: NumericType;
};
