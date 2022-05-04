import {
  FoxgloveEnumSchema,
  FoxgloveMessageSchema,
  FoxglovePrimitive,
} from "./types";

const allPrimitives: FoxglovePrimitive[] = [
  "Duration",
  "Time",
  "boolean",
  "bytes",
  "float",
  "integer",
  "string",
];

export const exampleEnum: FoxgloveEnumSchema = {
  type: "enum",
  name: "ExampleEnum",
  description: "An example enum",
  values: [
    { name: "A", value: 1, description: "Value A" },
    { name: "B", value: 2, description: "Value B" },
  ],
};

const exampleNestedMessage: FoxgloveMessageSchema = {
  type: "message",
  name: "NestedMessage",
  description: "An example nested message",
  fields: [
    {
      name: "field_enum",
      description: "An enum field",
      type: { type: "primitive", name: "integer" },
    },
  ],
};

export const exampleMessage: FoxgloveMessageSchema = {
  type: "message",
  name: "ExampleMessage",
  description: "An example type",
  fields: [
    ...allPrimitives.map((name): FoxgloveMessageSchema["fields"][0] => ({
      name: `field_${name.toLowerCase()}`,
      description: `${name} field`,
      type: { type: "primitive", name },
    })),
    ...allPrimitives.map((name): FoxgloveMessageSchema["fields"][0] => ({
      name: `field_${name.toLowerCase()}_array`,
      description: `${name} array field`,
      type: { type: "primitive", name },
      array: true,
    })),
    {
      name: "field_enum",
      description: "An enum field",
      type: { type: "enum", enum: exampleEnum },
    },
    {
      name: "field_enum_array",
      description: "An enum array field",
      type: { type: "enum", enum: exampleEnum },
      array: true,
    },
    {
      name: "field_nested",
      description: "A nested field",
      type: { type: "nested", schema: exampleNestedMessage },
    },
    {
      name: "field_nested_array",
      description: "A nested array field",
      type: { type: "nested", schema: exampleNestedMessage },
      array: true,
    },
  ],
};

export const exampleMessageWithoutArrayOfBytes: FoxgloveMessageSchema = {
  ...exampleMessage,
  fields: exampleMessage.fields.filter(
    ({ name }) => name !== "field_bytes_array"
  ),
};
