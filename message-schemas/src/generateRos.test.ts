import { foxgloveSchemas } from ".";
import { generateRosMsgMergedSchema, generateRosMsgFiles } from "./generateRos";

describe("generateRosMsgFiles", () => {
  it("generates msg files", () => {
    expect(generateRosMsgFiles(foxgloveSchemas["foxglove.Markers.LineMarker"]))
      .toMatchInlineSnapshot(`
      Array [
        Object {
          "filename": "foxglove.Markers.LineMarker.msg",
          "name": "foxglove.Markers.LineMarker",
          "source": "# Generated from foxglove.Markers.LineMarker by @foxglove/message-schemas

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
      KeyValuePair[] metadata

      # Origin of lines relative to reference frame
      Pose pose

      # Line thickness
      float64 thickness

      # Indicates whether \`thickness\` is a fixed size in screen pixels (true), or specified in world coordinates and scales with distance from the camera (false)
      bool scale_invariant

      # Points along the line
      Point3[] points

      # Solid color to use for the whole line. One of \`color\` or \`colors\` must be provided.
      Color color

      # Per-point colors (if specified, must have the same length as \`points\`). One of \`color\` or \`colors\` must be provided.
      Color[] colors

      # Indexes into the \`points\` and \`colors\` attribute arrays, which can be used to avoid duplicating attribute data.
      int32[] indices
      ",
        },
      ]
    `);
  });
});

describe("generateRosMsgMergedSchema", () => {
  it("generates merged schema", () => {
    expect(
      generateRosMsgMergedSchema(foxgloveSchemas["foxglove.Markers.LineMarker"])
    ).toMatchInlineSnapshot(`
      "# Generated from foxglove.Markers.LineMarker by @foxglove/message-schemas

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
      KeyValuePair[] metadata

      # Origin of lines relative to reference frame
      Pose pose

      # Line thickness
      float64 thickness

      # Indicates whether \`thickness\` is a fixed size in screen pixels (true), or specified in world coordinates and scales with distance from the camera (false)
      bool scale_invariant

      # Points along the line
      Point3[] points

      # Solid color to use for the whole line. One of \`color\` or \`colors\` must be provided.
      Color color

      # Per-point colors (if specified, must have the same length as \`points\`). One of \`color\` or \`colors\` must be provided.
      Color[] colors

      # Indexes into the \`points\` and \`colors\` attribute arrays, which can be used to avoid duplicating attribute data.
      int32[] indices
      "
    `);
  });
});
