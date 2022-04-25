export type FoxglovePrimitive =
  | "string"
  | "float"
  | "integer"
  | "boolean"
  | "bytes"
  | "Time"
  | "Duration";

export type FoxgloveSchema = {
  name: string;
  enums?: ReadonlyArray<{
    name: string;
    values: ReadonlyArray<{
      value: number;
      name: string;
      description?: string;
    }>;
  }>;
  fields: ReadonlyArray<{
    name: string;
    type:
      | { type: "primitive"; name: FoxglovePrimitive }
      | { type: "nested"; name: string }
      | { type: "enum"; name: string };
    array?: boolean;
    required?: true;
    description: string;
  }>;
};
