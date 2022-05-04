// Generated from DataField by @foxglove/message-schemas

import { NumericType } from "./NumericType";

/** List of fields included for every entity in an accompanying `data` field array */
export type DataField = {
  /** Name of the field */
  name: string;

  /** Byte offset from start of data buffer */
  offset: number;

  /** Type of data in field */
  type: NumericType;
};
