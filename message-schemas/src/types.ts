export type FoxglovePrimitive =
  | "string"
  | "float"
  | "integer"
  | "boolean"
  | "bytes"
  | "Time"
  | "Duration";

export type FoxgloveEnum = {
  name: string;
  values: ReadonlyArray<{
    value: number;
    name: string;
    description?: string;
  }>;
};

export type FoxgloveSchema = {
  name: string;
  rosEquivalent?: keyof typeof import("@foxglove/rosmsg-msgs-common").definitions;
  enums?: ReadonlyArray<FoxgloveEnum>;
  fields: ReadonlyArray<{
    name: string;
    type:
      | { type: "primitive"; name: FoxglovePrimitive }
      | { type: "nested"; schema: FoxgloveSchema }
      | { type: "enum"; enum: FoxgloveEnum };
    array?: boolean;
    required?: true;
    description: string;
  }>;
};
