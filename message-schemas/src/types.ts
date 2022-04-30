export type FoxglovePrimitive =
  | "string"
  | "float"
  | "integer"
  | "boolean"
  | "bytes"
  | "Time"
  | "Duration";

export type FoxgloveEnumSchema = {
  type: "enum";
  name: string;
  values: ReadonlyArray<{
    value: number;
    name: string;
    description?: string;
  }>;
};

export type FoxgloveMessageSchema = {
  type: "message";
  name: string;
  rosEquivalent?: keyof typeof import("@foxglove/rosmsg-msgs-common").definitions;
  fields: ReadonlyArray<{
    name: string;
    type:
      | { type: "primitive"; name: FoxglovePrimitive }
      | { type: "nested"; schema: FoxgloveMessageSchema }
      | { type: "enum"; enum: FoxgloveEnumSchema };
    array?: boolean;
    required?: true;
    description: string;
  }>;
};

export type FoxgloveSchema = FoxgloveMessageSchema | FoxgloveEnumSchema;
