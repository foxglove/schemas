export type FoxglovePrimitive =
  | "string"
  | "float64"
  | "uint32"
  | "boolean"
  | "bytes"
  | "time"
  | "duration";

export type FoxgloveEnumSchema = {
  type: "enum";
  name: string;
  description: string;
  protobufParentMessageName: string;
  protobufEnumName: string;
  values: ReadonlyArray<{
    value: number;
    name: string;
    description?: string;
  }>;
};

export type FoxgloveMessageSchema = {
  type: "message";
  name: string;
  description: string;
  rosEquivalent?: keyof typeof import("@foxglove/rosmsg-msgs-common").definitions;
  fields: ReadonlyArray<{
    name: string;
    type:
      | { type: "primitive"; name: FoxglovePrimitive }
      | { type: "nested"; schema: FoxgloveMessageSchema }
      | { type: "enum"; enum: FoxgloveEnumSchema };
    array?: true | number;
    required?: true;
    description: string;
  }>;
};

export type FoxgloveSchema = FoxgloveMessageSchema | FoxgloveEnumSchema;
