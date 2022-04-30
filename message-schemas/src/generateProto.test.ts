import protobufjs from "protobufjs";

import { generateProto } from "./generateProto";
import { foxgloveMessageSchemas } from "./schemas";

describe("generateProtoFiles", () => {
  it("generates .proto files", () => {
    expect(generateProto(foxgloveMessageSchemas["LineMarker"]))
      .toMatchInlineSnapshot(`
      "// Generated from LineMarker by @foxglove/message-schemas

      syntax = \\"proto3\\";

      import \\"foxglove/Color.proto\\";
      import \\"foxglove/KeyValuePair.proto\\";
      import \\"foxglove/LineType.proto\\";
      import \\"foxglove/Point3.proto\\";
      import \\"foxglove/Pose.proto\\";
      import \\"foxglove/builtins.proto\\";

      package foxglove;

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

    expect(() =>
      protobufjs.parse(generateProto(foxgloveMessageSchemas["LineMarker"]))
    ).not.toThrow();
  });
});
