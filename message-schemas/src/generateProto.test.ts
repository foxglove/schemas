import protobufjs from "protobufjs";

import { DURATION_PROTO, generateProto, TIME_PROTO } from "./generateProto";
import { foxgloveEnumSchemas, foxgloveMessageSchemas } from "./schemas";

describe("generateProtoFiles", () => {
  it("generates .proto files", () => {
    expect(generateProto(foxgloveMessageSchemas["LineMarker"]))
      .toMatchInlineSnapshot(`
      "// Generated from LineMarker by @foxglove/message-schemas

      syntax = \\"proto3\\";

      import \\"foxglove/Color.proto\\";
      import \\"foxglove/Duration.proto\\";
      import \\"foxglove/KeyValuePair.proto\\";
      import \\"foxglove/LineType.proto\\";
      import \\"foxglove/Point3.proto\\";
      import \\"foxglove/Pose.proto\\";
      import \\"foxglove/Time.proto\\";

      package foxglove;

      // A marker representing a series of points connected by lines
      message LineMarker {
        // Timestamp of the marker
        foxglove.Time timestamp = 1;

        // Frame of reference
        string frame_id = 2;

        // Namespace into which the marker should be grouped. A marker will replace any prior marker on the same topic with the same \`namespace\` and \`id\`.
        string namespace = 3;

        // Identifier for the marker. A marker will replace any prior marker on the same topic with the same \`namespace\` and \`id\`.
        string id = 4;

        // Length of time (relative to \`timestamp\`) after which the marker should be automatically removed. Zero value indicates the marker should remain visible until it is replaced or deleted.
        foxglove.Duration lifetime = 5;

        // Whether the marker should keep its location in the fixed frame (false) or follow the frame specified in \`frame_id\` as it moves relative to the fixed frame (true)
        bool frame_locked = 6;

        // Additional user-provided metadata associated with the marker. Keys must be unique.
        repeated foxglove.KeyValuePair metadata = 7;

        // Drawing primitive to use for lines
        foxglove.LineType type = 8;

        // Origin of lines relative to reference frame
        foxglove.Pose pose = 9;

        // Line thickness
        double thickness = 10;

        // Indicates whether \`thickness\` is a fixed size in screen pixels (true), or specified in world coordinates and scales with distance from the camera (false)
        bool scale_invariant = 11;

        // Points along the line
        repeated foxglove.Point3 points = 12;

        // Solid color to use for the whole line. One of \`color\` or \`colors\` must be provided.
        foxglove.Color color = 13;

        // Per-point colors (if specified, must have the same length as \`points\`). One of \`color\` or \`colors\` must be provided.
        repeated foxglove.Color colors = 14;

        // Indexes into the \`points\` and \`colors\` attribute arrays, which can be used to avoid duplicating attribute data.
        repeated int32 indices = 15;
      }
      "
    `);
  });

  it("generates parseable .proto files", () => {
    const root = new protobufjs.Root();
    root.add(protobufjs.parse(TIME_PROTO).root);
    root.add(protobufjs.parse(DURATION_PROTO).root);
    for (const schema of Object.values(foxgloveMessageSchemas)) {
      root.add(protobufjs.parse(generateProto(schema)).root);
    }
    for (const schema of Object.values(foxgloveEnumSchemas)) {
      root.add(protobufjs.parse(generateProto(schema)).root);
    }
    expect(() => root.resolveAll()).not.toThrow();
  });
});
