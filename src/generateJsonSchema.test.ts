import Ajv from "ajv";

import { generateJsonSchema } from "./generateJsonSchema";
import { foxgloveMessageSchemas } from "./schemas";
import { exampleMessage } from "./testFixtures";

describe("generateJsonSchema", () => {
  it("generates expected JSON Schema", () => {
    expect(generateJsonSchema(exampleMessage)).toMatchInlineSnapshot(`
      Object {
        "$comment": "Generated from ExampleMessage by @foxglove/message-schemas",
        "description": "An example type",
        "properties": Object {
          "field_Duration": Object {
            "description": "Duration field",
            "properties": Object {
              "nsec": Object {
                "maximum": 999999999,
                "minimum": 0,
                "type": "integer",
              },
              "sec": Object {
                "type": "integer",
              },
            },
            "title": "Duration",
            "type": "object",
          },
          "field_Duration_array": Object {
            "description": "Duration array field",
            "items": Object {
              "properties": Object {
                "nsec": Object {
                  "maximum": 999999999,
                  "minimum": 0,
                  "type": "integer",
                },
                "sec": Object {
                  "type": "integer",
                },
              },
              "title": "Duration",
              "type": "object",
            },
            "type": "array",
          },
          "field_Duration_fixed_array": Object {
            "description": "Duration fixed-length array field",
            "items": Object {
              "properties": Object {
                "nsec": Object {
                  "maximum": 999999999,
                  "minimum": 0,
                  "type": "integer",
                },
                "sec": Object {
                  "type": "integer",
                },
              },
              "title": "Duration",
              "type": "object",
            },
            "maxItems": 3,
            "minItems": 3,
            "type": "array",
          },
          "field_Time": Object {
            "description": "Time field",
            "properties": Object {
              "nsec": Object {
                "maximum": 999999999,
                "minimum": 0,
                "type": "integer",
              },
              "sec": Object {
                "minimum": 0,
                "type": "integer",
              },
            },
            "title": "Time",
            "type": "object",
          },
          "field_Time_array": Object {
            "description": "Time array field",
            "items": Object {
              "properties": Object {
                "nsec": Object {
                  "maximum": 999999999,
                  "minimum": 0,
                  "type": "integer",
                },
                "sec": Object {
                  "minimum": 0,
                  "type": "integer",
                },
              },
              "title": "Time",
              "type": "object",
            },
            "type": "array",
          },
          "field_Time_fixed_array": Object {
            "description": "Time fixed-length array field",
            "items": Object {
              "properties": Object {
                "nsec": Object {
                  "maximum": 999999999,
                  "minimum": 0,
                  "type": "integer",
                },
                "sec": Object {
                  "minimum": 0,
                  "type": "integer",
                },
              },
              "title": "Time",
              "type": "object",
            },
            "maxItems": 3,
            "minItems": 3,
            "type": "array",
          },
          "field_boolean": Object {
            "description": "boolean field",
            "type": "boolean",
          },
          "field_boolean_array": Object {
            "description": "boolean array field",
            "items": Object {
              "type": "boolean",
            },
            "type": "array",
          },
          "field_boolean_fixed_array": Object {
            "description": "boolean fixed-length array field",
            "items": Object {
              "type": "boolean",
            },
            "maxItems": 3,
            "minItems": 3,
            "type": "array",
          },
          "field_bytes": Object {
            "contentEncoding": "base64",
            "description": "bytes field",
            "type": "string",
          },
          "field_bytes_array": Object {
            "description": "bytes array field",
            "items": Object {
              "contentEncoding": "base64",
              "type": "string",
            },
            "type": "array",
          },
          "field_bytes_fixed_array": Object {
            "description": "bytes fixed-length array field",
            "items": Object {
              "contentEncoding": "base64",
              "type": "string",
            },
            "maxItems": 3,
            "minItems": 3,
            "type": "array",
          },
          "field_enum": Object {
            "description": "An enum field",
            "oneOf": Array [
              Object {
                "const": 1,
                "description": "Value A",
                "title": "A",
              },
              Object {
                "const": 2,
                "description": "Value B",
                "title": "B",
              },
            ],
            "title": "ExampleEnum: An example enum",
          },
          "field_enum_array": Object {
            "description": "An enum array field",
            "items": Object {
              "description": "An enum array field",
              "oneOf": Array [
                Object {
                  "const": 1,
                  "description": "Value A",
                  "title": "A",
                },
                Object {
                  "const": 2,
                  "description": "Value B",
                  "title": "B",
                },
              ],
              "title": "ExampleEnum: An example enum",
            },
            "type": "array",
          },
          "field_float": Object {
            "description": "float field",
            "type": "number",
          },
          "field_float_array": Object {
            "description": "float array field",
            "items": Object {
              "type": "number",
            },
            "type": "array",
          },
          "field_float_fixed_array": Object {
            "description": "float fixed-length array field",
            "items": Object {
              "type": "number",
            },
            "maxItems": 3,
            "minItems": 3,
            "type": "array",
          },
          "field_nested": Object {
            "$comment": "Generated from NestedMessage by @foxglove/message-schemas",
            "description": "A nested field",
            "properties": Object {
              "field_enum": Object {
                "description": "An enum field",
                "minimum": 0,
                "type": "integer",
              },
            },
            "title": "NestedMessage",
            "type": "object",
          },
          "field_nested_array": Object {
            "description": "A nested array field
      With
      a
      very
      long
      description",
            "items": Object {
              "$comment": "Generated from NestedMessage by @foxglove/message-schemas",
              "description": "An example nested message",
              "properties": Object {
                "field_enum": Object {
                  "description": "An enum field",
                  "minimum": 0,
                  "type": "integer",
                },
              },
              "title": "NestedMessage",
              "type": "object",
            },
            "type": "array",
          },
          "field_string": Object {
            "description": "string field",
            "type": "string",
          },
          "field_string_array": Object {
            "description": "string array field",
            "items": Object {
              "type": "string",
            },
            "type": "array",
          },
          "field_string_fixed_array": Object {
            "description": "string fixed-length array field",
            "items": Object {
              "type": "string",
            },
            "maxItems": 3,
            "minItems": 3,
            "type": "array",
          },
          "field_uint32": Object {
            "description": "uint32 field",
            "minimum": 0,
            "type": "integer",
          },
          "field_uint32_array": Object {
            "description": "uint32 array field",
            "items": Object {
              "minimum": 0,
              "type": "integer",
            },
            "type": "array",
          },
          "field_uint32_fixed_array": Object {
            "description": "uint32 fixed-length array field",
            "items": Object {
              "minimum": 0,
              "type": "integer",
            },
            "maxItems": 3,
            "minItems": 3,
            "type": "array",
          },
        },
        "title": "ExampleMessage",
        "type": "object",
      }
    `);
  });

  it.each(Object.values(foxgloveMessageSchemas))(
    "generates parseable JSON Schema for $name",
    (schema) => {
      const ajv = new Ajv();
      expect(() => ajv.compile(generateJsonSchema(schema))).not.toThrow();
    },
  );
});
