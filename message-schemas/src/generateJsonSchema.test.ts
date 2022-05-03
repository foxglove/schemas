import Ajv from "ajv";

import { generateJsonSchema } from "./generateJsonSchema";
import { foxgloveMessageSchemas } from "./schemas";

describe("generateJsonSchema", () => {
  it("generates expected JSON Schema", () => {
    expect(generateJsonSchema(foxgloveMessageSchemas["LineMarker"]))
      .toMatchInlineSnapshot(`
      Object {
        "$comment": "Generated from LineMarker by @foxglove/message-schemas",
        "properties": Object {
          "color": Object {
            "$comment": "Generated from Color by @foxglove/message-schemas",
            "description": "Solid color to use for the whole line. One of \`color\` or \`colors\` must be provided.",
            "properties": Object {
              "a": Object {
                "description": "Alpha value between 0 and 1",
                "type": "number",
              },
              "b": Object {
                "description": "Blue value between 0 and 1",
                "type": "number",
              },
              "g": Object {
                "description": "Green value between 0 and 1",
                "type": "number",
              },
              "r": Object {
                "description": "Red value between 0 and 1",
                "type": "number",
              },
            },
            "type": "object",
          },
          "colors": Object {
            "items": Object {
              "$comment": "Generated from Color by @foxglove/message-schemas",
              "description": "Per-point colors (if specified, must have the same length as \`points\`). One of \`color\` or \`colors\` must be provided.",
              "properties": Object {
                "a": Object {
                  "description": "Alpha value between 0 and 1",
                  "type": "number",
                },
                "b": Object {
                  "description": "Blue value between 0 and 1",
                  "type": "number",
                },
                "g": Object {
                  "description": "Green value between 0 and 1",
                  "type": "number",
                },
                "r": Object {
                  "description": "Red value between 0 and 1",
                  "type": "number",
                },
              },
              "type": "object",
            },
            "type": "array",
          },
          "frame_id": Object {
            "description": "Frame of reference",
            "type": "string",
          },
          "frame_locked": Object {
            "description": "Whether the marker should keep its location in the fixed frame (false) or follow the frame specified in \`frame_id\` as it moves relative to the fixed frame (true)",
            "type": "boolean",
          },
          "id": Object {
            "description": "Identifier for the marker. A marker will replace any prior marker on the same topic with the same \`namespace\` and \`id\`.",
            "type": "string",
          },
          "indices": Object {
            "items": Object {
              "description": "Indexes into the \`points\` and \`colors\` attribute arrays, which can be used to avoid duplicating attribute data.",
              "type": "number",
            },
            "type": "array",
          },
          "lifetime": Object {
            "description": "Length of time (relative to \`timestamp\`) after which the marker should be automatically removed. Zero value indicates the marker should remain visible until it is replaced or deleted.",
            "properties": Object {
              "nsec": Object {
                "type": "integer",
              },
              "sec": Object {
                "type": "integer",
              },
            },
            "type": "object",
          },
          "metadata": Object {
            "items": Object {
              "$comment": "Generated from KeyValuePair by @foxglove/message-schemas",
              "description": "Additional user-provided metadata associated with the marker. Keys must be unique.",
              "properties": Object {
                "key": Object {
                  "description": "Key",
                  "type": "string",
                },
                "value": Object {
                  "description": "Value",
                  "type": "string",
                },
              },
              "type": "object",
            },
            "type": "array",
          },
          "namespace": Object {
            "description": "Namespace into which the marker should be grouped. A marker will replace any prior marker on the same topic with the same \`namespace\` and \`id\`.",
            "type": "string",
          },
          "points": Object {
            "items": Object {
              "$comment": "Generated from Point3 by @foxglove/message-schemas",
              "description": "Points along the line",
              "properties": Object {
                "x": Object {
                  "description": "x coordinate position",
                  "type": "number",
                },
                "y": Object {
                  "description": "y coordinate position",
                  "type": "number",
                },
                "z": Object {
                  "description": "z coordinate position",
                  "type": "number",
                },
              },
              "type": "object",
            },
            "type": "array",
          },
          "pose": Object {
            "$comment": "Generated from Pose by @foxglove/message-schemas",
            "description": "Origin of lines relative to reference frame",
            "properties": Object {
              "orientation": Object {
                "$comment": "Generated from Quaternion by @foxglove/message-schemas",
                "description": "Quaternion denoting orientation in 3D space",
                "properties": Object {
                  "w": Object {
                    "description": "w value",
                    "type": "number",
                  },
                  "x": Object {
                    "description": "x value",
                    "type": "number",
                  },
                  "y": Object {
                    "description": "y value",
                    "type": "number",
                  },
                  "z": Object {
                    "description": "z value",
                    "type": "number",
                  },
                },
                "type": "object",
              },
              "position": Object {
                "$comment": "Generated from Vector3 by @foxglove/message-schemas",
                "description": "Point denoting position in 3D space",
                "properties": Object {
                  "x": Object {
                    "description": "x coordinate length",
                    "type": "number",
                  },
                  "y": Object {
                    "description": "y coordinate length",
                    "type": "number",
                  },
                  "z": Object {
                    "description": "z coordinate length",
                    "type": "number",
                  },
                },
                "type": "object",
              },
            },
            "type": "object",
          },
          "scale_invariant": Object {
            "description": "Indicates whether \`thickness\` is a fixed size in screen pixels (true), or specified in world coordinates and scales with distance from the camera (false)",
            "type": "boolean",
          },
          "thickness": Object {
            "description": "Line thickness",
            "type": "number",
          },
          "timestamp": Object {
            "description": "Timestamp of the marker",
            "properties": Object {
              "nsec": Object {
                "type": "integer",
              },
              "sec": Object {
                "type": "integer",
              },
            },
            "type": "object",
          },
          "type": Object {
            "description": "Drawing primitive to use for lines",
            "oneOf": Array [
              Object {
                "const": 0,
                "description": "0-1, 1-2, ...",
                "title": "LINE_STRIP",
              },
              Object {
                "const": 1,
                "description": "0-1, 1-2, ..., n-0",
                "title": "LINE_LOOP",
              },
              Object {
                "const": 2,
                "description": "0-1, 2-3, 4-5, ...",
                "title": "LINE_LIST",
              },
            ],
          },
        },
        "type": "object",
      }
    `);
  });

  it.each(Object.values(foxgloveMessageSchemas))(
    "generates parseable JSON Schema for $name",
    (schema) => {
      const ajv = new Ajv();
      expect(() => ajv.compile(generateJsonSchema(schema))).not.toThrow();
    }
  );
});
