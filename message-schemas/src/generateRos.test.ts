import { parse as parseMessageDefinition } from "@foxglove/rosmsg";

import { generateRosMsgMergedSchema, generateRosMsgFiles } from "./generateRos";
import { foxgloveMessageSchemas } from "./schemas";

describe("generateRosMsgFiles", () => {
  it("generates msg files", () => {
    expect(generateRosMsgFiles(foxgloveMessageSchemas["LineMarker"]))
      .toMatchInlineSnapshot(`
      Array [
        Object {
          "filename": "foxglove_msgs/LineMarker.msg",
          "name": "foxglove_msgs/LineMarker",
          "source": "# Generated from LineMarker by @foxglove/message-schemas

      # Timestamp of the marker
      time timestamp

      # Frame of reference
      string frame_id

      # Namespace into which the marker should be grouped. A marker will replace any prior marker on the same topic with the same \`namespace\` and \`id\`.
      string namespace

      # Identifier for the marker. A marker will replace any prior marker on the same topic with the same \`namespace\` and \`id\`.
      string id

      # Length of time (relative to \`timestamp\`) after which the marker should be automatically removed. Zero value indicates the marker should remain visible until it is replaced or deleted.
      duration lifetime

      # Whether the marker should keep its location in the fixed frame (false) or follow the frame specified in \`frame_id\` as it moves relative to the fixed frame (true)
      bool frame_locked

      # Additional user-provided metadata associated with the marker. Keys must be unique.
      foxglove_msgs/KeyValuePair[] metadata

      # 0-1, 1-2, ...
      uint8 LINE_STRIP=0

      # 0-1, 1-2, ..., n-0
      uint8 LINE_LOOP=1

      # 0-1, 2-3, 4-5, ...
      uint8 LINE_LIST=2

      # Drawing primitive to use for lines
      uint8 type

      # Origin of lines relative to reference frame
      geometry_msgs/Pose pose

      # Line thickness
      float64 thickness

      # Indicates whether \`thickness\` is a fixed size in screen pixels (true), or specified in world coordinates and scales with distance from the camera (false)
      bool scale_invariant

      # Points along the line
      geometry_msgs/Point[] points

      # Solid color to use for the whole line. One of \`color\` or \`colors\` must be provided.
      foxglove_msgs/Color color

      # Per-point colors (if specified, must have the same length as \`points\`). One of \`color\` or \`colors\` must be provided.
      foxglove_msgs/Color[] colors

      # Indexes into the \`points\` and \`colors\` attribute arrays, which can be used to avoid duplicating attribute data.
      int32[] indices
      ",
        },
        Object {
          "filename": "foxglove_msgs/Color.msg",
          "name": "foxglove_msgs/Color",
          "source": "# Generated from Color by @foxglove/message-schemas

      # Red value between 0 and 1
      float64 r

      # Green value between 0 and 1
      float64 g

      # Blue value between 0 and 1
      float64 b

      # Alpha value between 0 and 1
      float64 a
      ",
        },
        Object {
          "filename": "foxglove_msgs/Color.msg",
          "name": "foxglove_msgs/Color",
          "source": "# Generated from Color by @foxglove/message-schemas

      # Red value between 0 and 1
      float64 r

      # Green value between 0 and 1
      float64 g

      # Blue value between 0 and 1
      float64 b

      # Alpha value between 0 and 1
      float64 a
      ",
        },
        Object {
          "filename": "geometry_msgs/Point.msg",
          "name": "geometry_msgs/Point",
          "source": "# Generated from geometry_msgs/Point by @foxglove/message-schemas
      float64 x
      float64 y
      float64 z
      ",
        },
        Object {
          "filename": "geometry_msgs/Quaternion.msg",
          "name": "geometry_msgs/Quaternion",
          "source": "# Generated from geometry_msgs/Quaternion by @foxglove/message-schemas
      float64 x
      float64 y
      float64 z
      float64 w
      ",
        },
        Object {
          "filename": "geometry_msgs/Point.msg",
          "name": "geometry_msgs/Point",
          "source": "# Generated from geometry_msgs/Point by @foxglove/message-schemas
      float64 x
      float64 y
      float64 z
      ",
        },
        Object {
          "filename": "geometry_msgs/Pose.msg",
          "name": "geometry_msgs/Pose",
          "source": "# Generated from geometry_msgs/Pose by @foxglove/message-schemas
      geometry_msgs/Point position
      geometry_msgs/Quaternion orientation
      ",
        },
        Object {
          "filename": "foxglove_msgs/KeyValuePair.msg",
          "name": "foxglove_msgs/KeyValuePair",
          "source": "# Generated from KeyValuePair by @foxglove/message-schemas

      # Key
      string key

      # Value
      string value
      ",
        },
      ]
    `);
  });
});

describe("generateRosMsgMergedSchema", () => {
  it("generates merged schema", () => {
    const mergedSchema = generateRosMsgMergedSchema(
      foxgloveMessageSchemas["LineMarker"]
    );
    expect(mergedSchema).toMatchInlineSnapshot(`
      "# Generated from LineMarker by @foxglove/message-schemas

      # Timestamp of the marker
      time timestamp

      # Frame of reference
      string frame_id

      # Namespace into which the marker should be grouped. A marker will replace any prior marker on the same topic with the same \`namespace\` and \`id\`.
      string namespace

      # Identifier for the marker. A marker will replace any prior marker on the same topic with the same \`namespace\` and \`id\`.
      string id

      # Length of time (relative to \`timestamp\`) after which the marker should be automatically removed. Zero value indicates the marker should remain visible until it is replaced or deleted.
      duration lifetime

      # Whether the marker should keep its location in the fixed frame (false) or follow the frame specified in \`frame_id\` as it moves relative to the fixed frame (true)
      bool frame_locked

      # Additional user-provided metadata associated with the marker. Keys must be unique.
      foxglove_msgs/KeyValuePair[] metadata

      # 0-1, 1-2, ...
      uint8 LINE_STRIP=0

      # 0-1, 1-2, ..., n-0
      uint8 LINE_LOOP=1

      # 0-1, 2-3, 4-5, ...
      uint8 LINE_LIST=2

      # Drawing primitive to use for lines
      uint8 type

      # Origin of lines relative to reference frame
      geometry_msgs/Pose pose

      # Line thickness
      float64 thickness

      # Indicates whether \`thickness\` is a fixed size in screen pixels (true), or specified in world coordinates and scales with distance from the camera (false)
      bool scale_invariant

      # Points along the line
      geometry_msgs/Point[] points

      # Solid color to use for the whole line. One of \`color\` or \`colors\` must be provided.
      foxglove_msgs/Color color

      # Per-point colors (if specified, must have the same length as \`points\`). One of \`color\` or \`colors\` must be provided.
      foxglove_msgs/Color[] colors

      # Indexes into the \`points\` and \`colors\` attribute arrays, which can be used to avoid duplicating attribute data.
      int32[] indices
      ================================================================================
      MSG: foxglove_msgs/Color
      # Generated from Color by @foxglove/message-schemas

      # Red value between 0 and 1
      float64 r

      # Green value between 0 and 1
      float64 g

      # Blue value between 0 and 1
      float64 b

      # Alpha value between 0 and 1
      float64 a
      ================================================================================
      MSG: foxglove_msgs/Color
      # Generated from Color by @foxglove/message-schemas

      # Red value between 0 and 1
      float64 r

      # Green value between 0 and 1
      float64 g

      # Blue value between 0 and 1
      float64 b

      # Alpha value between 0 and 1
      float64 a
      ================================================================================
      MSG: geometry_msgs/Point
      # Generated from geometry_msgs/Point by @foxglove/message-schemas
      float64 x
      float64 y
      float64 z
      ================================================================================
      MSG: geometry_msgs/Quaternion
      # Generated from geometry_msgs/Quaternion by @foxglove/message-schemas
      float64 x
      float64 y
      float64 z
      float64 w
      ================================================================================
      MSG: geometry_msgs/Point
      # Generated from geometry_msgs/Point by @foxglove/message-schemas
      float64 x
      float64 y
      float64 z
      ================================================================================
      MSG: geometry_msgs/Pose
      # Generated from geometry_msgs/Pose by @foxglove/message-schemas
      geometry_msgs/Point position
      geometry_msgs/Quaternion orientation
      ================================================================================
      MSG: foxglove_msgs/KeyValuePair
      # Generated from KeyValuePair by @foxglove/message-schemas

      # Key
      string key

      # Value
      string value
      "
    `);
    expect(parseMessageDefinition(mergedSchema)).toMatchInlineSnapshot(`
      Array [
        Object {
          "definitions": Array [
            Object {
              "isArray": false,
              "isComplex": false,
              "name": "timestamp",
              "type": "time",
            },
            Object {
              "isArray": false,
              "isComplex": false,
              "name": "frame_id",
              "type": "string",
            },
            Object {
              "isArray": false,
              "isComplex": false,
              "name": "namespace",
              "type": "string",
            },
            Object {
              "isArray": false,
              "isComplex": false,
              "name": "id",
              "type": "string",
            },
            Object {
              "isArray": false,
              "isComplex": false,
              "name": "lifetime",
              "type": "duration",
            },
            Object {
              "isArray": false,
              "isComplex": false,
              "name": "frame_locked",
              "type": "bool",
            },
            Object {
              "isArray": true,
              "isComplex": true,
              "name": "metadata",
              "type": "foxglove_msgs/KeyValuePair",
            },
            Object {
              "isConstant": true,
              "name": "LINE_STRIP",
              "type": "uint8",
              "value": 0,
              "valueText": "0",
            },
            Object {
              "isConstant": true,
              "name": "LINE_LOOP",
              "type": "uint8",
              "value": 1,
              "valueText": "1",
            },
            Object {
              "isConstant": true,
              "name": "LINE_LIST",
              "type": "uint8",
              "value": 2,
              "valueText": "2",
            },
            Object {
              "isArray": false,
              "isComplex": false,
              "name": "type",
              "type": "uint8",
            },
            Object {
              "isArray": false,
              "isComplex": true,
              "name": "pose",
              "type": "geometry_msgs/Pose",
            },
            Object {
              "isArray": false,
              "isComplex": false,
              "name": "thickness",
              "type": "float64",
            },
            Object {
              "isArray": false,
              "isComplex": false,
              "name": "scale_invariant",
              "type": "bool",
            },
            Object {
              "isArray": true,
              "isComplex": true,
              "name": "points",
              "type": "geometry_msgs/Point",
            },
            Object {
              "isArray": false,
              "isComplex": true,
              "name": "color",
              "type": "foxglove_msgs/Color",
            },
            Object {
              "isArray": true,
              "isComplex": true,
              "name": "colors",
              "type": "foxglove_msgs/Color",
            },
            Object {
              "isArray": true,
              "isComplex": false,
              "name": "indices",
              "type": "int32",
            },
          ],
          "name": undefined,
        },
        Object {
          "definitions": Array [
            Object {
              "isArray": false,
              "isComplex": false,
              "name": "r",
              "type": "float64",
            },
            Object {
              "isArray": false,
              "isComplex": false,
              "name": "g",
              "type": "float64",
            },
            Object {
              "isArray": false,
              "isComplex": false,
              "name": "b",
              "type": "float64",
            },
            Object {
              "isArray": false,
              "isComplex": false,
              "name": "a",
              "type": "float64",
            },
          ],
          "name": "foxglove_msgs/Color",
        },
        Object {
          "definitions": Array [
            Object {
              "isArray": false,
              "isComplex": false,
              "name": "r",
              "type": "float64",
            },
            Object {
              "isArray": false,
              "isComplex": false,
              "name": "g",
              "type": "float64",
            },
            Object {
              "isArray": false,
              "isComplex": false,
              "name": "b",
              "type": "float64",
            },
            Object {
              "isArray": false,
              "isComplex": false,
              "name": "a",
              "type": "float64",
            },
          ],
          "name": "foxglove_msgs/Color",
        },
        Object {
          "definitions": Array [
            Object {
              "isArray": false,
              "isComplex": false,
              "name": "x",
              "type": "float64",
            },
            Object {
              "isArray": false,
              "isComplex": false,
              "name": "y",
              "type": "float64",
            },
            Object {
              "isArray": false,
              "isComplex": false,
              "name": "z",
              "type": "float64",
            },
          ],
          "name": "geometry_msgs/Point",
        },
        Object {
          "definitions": Array [
            Object {
              "isArray": false,
              "isComplex": false,
              "name": "x",
              "type": "float64",
            },
            Object {
              "isArray": false,
              "isComplex": false,
              "name": "y",
              "type": "float64",
            },
            Object {
              "isArray": false,
              "isComplex": false,
              "name": "z",
              "type": "float64",
            },
            Object {
              "isArray": false,
              "isComplex": false,
              "name": "w",
              "type": "float64",
            },
          ],
          "name": "geometry_msgs/Quaternion",
        },
        Object {
          "definitions": Array [
            Object {
              "isArray": false,
              "isComplex": false,
              "name": "x",
              "type": "float64",
            },
            Object {
              "isArray": false,
              "isComplex": false,
              "name": "y",
              "type": "float64",
            },
            Object {
              "isArray": false,
              "isComplex": false,
              "name": "z",
              "type": "float64",
            },
          ],
          "name": "geometry_msgs/Point",
        },
        Object {
          "definitions": Array [
            Object {
              "isArray": false,
              "isComplex": true,
              "name": "position",
              "type": "geometry_msgs/Point",
            },
            Object {
              "isArray": false,
              "isComplex": true,
              "name": "orientation",
              "type": "geometry_msgs/Quaternion",
            },
          ],
          "name": "geometry_msgs/Pose",
        },
        Object {
          "definitions": Array [
            Object {
              "isArray": false,
              "isComplex": false,
              "name": "key",
              "type": "string",
            },
            Object {
              "isArray": false,
              "isComplex": false,
              "name": "value",
              "type": "string",
            },
          ],
          "name": "foxglove_msgs/KeyValuePair",
        },
      ]
    `);
  });

  it.each(Object.values(foxgloveMessageSchemas))(
    "generates parseable merged schemas",
    (schema) => {
      expect(() =>
        parseMessageDefinition(generateRosMsgMergedSchema(schema))
      ).not.toThrow();
    }
  );
});
