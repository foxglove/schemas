import { generateJsonSchema } from "./generateJsonSchema";
import { foxgloveSchemas } from "./schemas";

describe("generateJsonSchema", () => {
  it("generates .proto files", () => {
    expect(generateJsonSchema(foxgloveSchemas["foxglove.Markers.LineMarker"]))
      .toMatchInlineSnapshot(`
      Object {
        "$comment": "Generated from foxglove.Markers.LineMarker by @foxglove/message-schemas",
        "properties": Object {
          "color": Object {
            "$comment": "Generated from Color by @foxglove/message-schemas",
            "properties": Object {
              "a": Object {
                "type": "number",
              },
              "b": Object {
                "type": "number",
              },
              "g": Object {
                "type": "number",
              },
              "r": Object {
                "type": "number",
              },
            },
            "type": "object",
          },
          "colors": Object {
            "items": Object {
              "$comment": "Generated from Color by @foxglove/message-schemas",
              "properties": Object {
                "a": Object {
                  "type": "number",
                },
                "b": Object {
                  "type": "number",
                },
                "g": Object {
                  "type": "number",
                },
                "r": Object {
                  "type": "number",
                },
              },
              "type": "object",
            },
            "type": "array",
          },
          "frame_id": Object {
            "type": "string",
          },
          "frame_locked": Object {
            "type": "boolean",
          },
          "id": Object {
            "type": "string",
          },
          "indices": Object {
            "items": Object {
              "type": "number",
            },
            "type": "array",
          },
          "lifetime": Object {
            "$comment": "originally duration",
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
              "properties": Object {
                "key": Object {
                  "type": "string",
                },
                "value": Object {
                  "type": "string",
                },
              },
              "type": "object",
            },
            "type": "array",
          },
          "namespace": Object {
            "type": "string",
          },
          "points": Object {
            "items": Object {
              "$comment": "Generated from Vector3 by @foxglove/message-schemas",
              "properties": Object {
                "x": Object {
                  "type": "number",
                },
                "y": Object {
                  "type": "number",
                },
                "z": Object {
                  "type": "number",
                },
              },
              "type": "object",
            },
            "type": "array",
          },
          "pose": Object {
            "$comment": "Generated from Pose by @foxglove/message-schemas",
            "properties": Object {
              "orientation": Object {
                "$comment": "Generated from Quaternion by @foxglove/message-schemas",
                "properties": Object {
                  "w": Object {
                    "type": "number",
                  },
                  "x": Object {
                    "type": "number",
                  },
                  "y": Object {
                    "type": "number",
                  },
                  "z": Object {
                    "type": "number",
                  },
                },
                "type": "object",
              },
              "position": Object {
                "$comment": "Generated from Vector3 by @foxglove/message-schemas",
                "properties": Object {
                  "x": Object {
                    "type": "number",
                  },
                  "y": Object {
                    "type": "number",
                  },
                  "z": Object {
                    "type": "number",
                  },
                },
                "type": "object",
              },
            },
            "type": "object",
          },
          "scale_invariant": Object {
            "type": "boolean",
          },
          "thickness": Object {
            "type": "number",
          },
          "timestamp": Object {
            "$comment": "originally time",
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
        },
        "type": "object",
      }
    `);
  });
});
