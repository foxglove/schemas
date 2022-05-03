import Ajv from "ajv";

import { generateJsonSchema } from "./generateJsonSchema";
import { foxgloveMessageSchemas } from "./schemas";

describe("generateJsonSchema", () => {
  it("generates expected JSON Schema", () => {
    expect(generateJsonSchema(foxgloveMessageSchemas["LineMarker"]))
      .toMatchInlineSnapshot(`
      Object {
        "$comment": "Generated from LineMarker by @foxglove/message-schemas",
        "description": "A marker representing a series of points connected by lines",
        "properties": Object {
          "color": Object {
            "$comment": "Generated from Color by @foxglove/message-schemas",
            "description": "A color in RGBA format",
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
            "title": "Color",
            "type": "object",
          },
          "colors": Object {
            "$comment": "Generated from Color by @foxglove/message-schemas",
            "description": "A color in RGBA format",
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
            "title": "Color",
            "type": "object",
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
            "description": "Indexes into the \`points\` and \`colors\` attribute arrays, which can be used to avoid duplicating attribute data.",
            "type": "number",
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
            "title": "Duration",
            "type": "object",
          },
          "metadata": Object {
            "$comment": "Generated from KeyValuePair by @foxglove/message-schemas",
            "description": "An entry representing a key and its associated value",
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
            "title": "KeyValuePair",
            "type": "object",
          },
          "namespace": Object {
            "description": "Namespace into which the marker should be grouped. A marker will replace any prior marker on the same topic with the same \`namespace\` and \`id\`.",
            "type": "string",
          },
          "points": Object {
            "$comment": "Generated from Point3 by @foxglove/message-schemas",
            "description": "A point representing a position in 3D space",
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
            "title": "Point3",
            "type": "object",
          },
          "pose": Object {
            "$comment": "Generated from Pose by @foxglove/message-schemas",
            "description": "The position and orientation of an object or reference frame in 3D space",
            "properties": Object {
              "orientation": Object {
                "$comment": "Generated from Quaternion by @foxglove/message-schemas",
                "description": "A [quaternion](https://eater.net/quaternions) representing a rotation in 3D space",
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
                "title": "Quaternion",
                "type": "object",
              },
              "position": Object {
                "$comment": "Generated from Vector3 by @foxglove/message-schemas",
                "description": "A vector in 3D space that represents a direction only",
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
                "title": "Vector3",
                "type": "object",
              },
            },
            "title": "Pose",
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
            "title": "Time",
            "type": "object",
          },
          "type": Object {
            "description": "Drawing primitive to use for lines",
            "oneOf": Array [
              Object {
                "const": 0,
                "description": "0-1, 1-2, ..., (n-1)-n",
                "title": "LINE_STRIP",
              },
              Object {
                "const": 1,
                "description": "0-1, 1-2, ..., (n-1)-n, n-0",
                "title": "LINE_LOOP",
              },
              Object {
                "const": 2,
                "description": "0-1, 2-3, 4-5, ...",
                "title": "LINE_LIST",
              },
            ],
            "title": "LineType: An enumeration indicating how input points should be interpreted to create lines",
          },
        },
        "title": "LineMarker",
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
