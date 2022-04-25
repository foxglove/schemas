import { FoxgloveSchema } from "./types";

/** Fields used in each Marker message */
const commonMarkerFields = [
  {
    name: "timestamp",
    type: { type: "primitive", name: "Time" },
    description: "Timestamp of the marker",
  },
  {
    name: "frame_id",
    type: { type: "primitive", name: "string" },
    description: "Frame of reference",
  },
  {
    name: "namespace",
    type: { type: "primitive", name: "string" },
    description:
      "Namespace into which the marker should be grouped. A marker will replace any prior marker on the same topic with the same `namespace` and `id`.",
  },
  {
    name: "id",
    type: { type: "primitive", name: "string" },
    description:
      "Identifier for the marker. A marker will replace any prior marker on the same topic with the same `namespace` and `id`.",
  },
  {
    name: "lifetime",
    type: { type: "primitive", name: "Duration" },
    description:
      "Length of time (relative to `timestamp`) after which the marker should be automatically removed. Zero value indicates the marker should remain visible until it is replaced or deleted.",
  },
  {
    name: "frame_locked",
    type: { type: "primitive", name: "boolean" },
    description:
      "Whether the marker should keep its location in the fixed frame (false) or follow the frame specified in `frame_id` as it moves relative to the fixed frame (true)",
  },
  {
    name: "metadata",
    type: { type: "nested", name: "KeyValuePair" },
    array: true,
    description:
      "Additional user-provided metadata associated with the marker. Keys must be unique.",
  },
] as const;

const foxgloveSchemas = {
  "foxglove.Markers": {
    name: "foxglove.Markers",
    fields: [
      {
        name: "deletions",
        type: { type: "nested", name: "foxglove.Markers.MarkerDeletion" },
        array: true,
        description: "Marker deletion actions",
      },
      {
        name: "arrows",
        type: { type: "nested", name: "foxglove.Markers.ArrowMarker" },
        array: true,
        description: "Arrow markers",
      },
      {
        name: "cubes",
        type: { type: "nested", name: "foxglove.Markers.CubeMarker" },
        array: true,
        description: "Cube markers",
      },
      {
        name: "spheres",
        type: { type: "nested", name: "foxglove.Markers.SphereMarker" },
        array: true,
        description: "Sphere markers",
      },
      {
        name: "cylinders",
        type: { type: "nested", name: "foxglove.Markers.CylinderMarker" },
        array: true,
        description: "Cylinder markers",
      },
      {
        name: "lines",
        type: { type: "nested", name: "foxglove.Markers.LineMarker" },
        array: true,
        description: "Line markers",
      },
      {
        name: "triangles",
        type: { type: "nested", name: "foxglove.Markers.TrianglesMarker" },
        array: true,
        description: "Triangles markers",
      },
      {
        name: "texts",
        type: { type: "nested", name: "foxglove.Markers.TextMarker" },
        array: true,
        description: "Text markers",
      },
      {
        name: "models",
        type: { type: "nested", name: "foxglove.Markers.ModelMarker" },
        array: true,
        description: "Model markers",
      },
    ],
  },

  KeyValuePair: {
    name: "KeyValuePair",
    fields: [
      {
        name: "key",
        type: { type: "primitive", name: "string" },
        description: "Key",
      },
      {
        name: "value",
        type: { type: "primitive", name: "string" },
        description: "Value",
      },
    ],
  },

  "foxglove.Markers.MarkerDeletion": {
    name: "foxglove.Markers.MarkerDeletion",
    enums: [
      {
        name: "DeleteKind",
        values: [
          { value: 0, name: "MATCHING_NAMESPACE_AND_ID" },
          { value: 1, name: "MATCHING_NAMESPACE" },
          { value: 2, name: "ALL" },
        ],
      },
    ],
    fields: [
      {
        name: "timestamp",
        type: { type: "primitive", name: "Time" },
        description:
          "Timestamp of the marker. Only matching markers earlier than this timestamp will be deleted.",
      },
      {
        name: "kind",
        type: { type: "enum", name: "DeleteKind" },
        description: "Type of deletion action to perform",
      },
      {
        name: "namespace",
        type: { type: "primitive", name: "string" },
        description:
          "Namespace which must match if `kind` is `MATCHING_NAMESPACE_AND_ID` or `MATCHING_NAMESPACE`.",
      },
      {
        name: "id",
        type: { type: "primitive", name: "string" },
        description:
          "Numeric identifier which must match if `kind` is `MATCHING_NAMESPACE_AND_ID`.",
      },
    ],
  },

  "foxglove.Markers.ArrowMarker": {
    name: "foxglove.Markers.ArrowMarker",
    fields: [
      ...commonMarkerFields,
      {
        name: "pose",
        type: { type: "nested", name: "Pose" },
        description:
          "Position of the arrow's tail and orientation of the arrow. Identity orientation means the arrow points in the +x direction.",
      },
      {
        name: "length",
        type: { type: "primitive", name: "float" },
        description: "Length of the arrow",
      },
      {
        name: "shaft_diameter",
        type: { type: "primitive", name: "float" },
        description: "Diameter of the arrow shaft",
      },
      {
        name: "head_diameter",
        type: { type: "primitive", name: "float" },
        description: "Diameter of the arrow head",
      },
      {
        name: "head_length",
        type: { type: "primitive", name: "float" },
        description: "Length of the arrow head",
      },
      {
        name: "color",
        type: { type: "nested", name: "Color" },
        description: "Color of the arrow",
      },
    ],
  },

  "foxglove.Markers.CubeMarker": {
    name: "foxglove.Markers.CubeMarker",
    fields: [
      ...commonMarkerFields,
      {
        name: "pose",
        type: { type: "nested", name: "Pose" },
        description:
          "Position of the center of the cube and orientation of the cube",
      },
      {
        name: "size",
        type: { type: "nested", name: "Vector3" },
        description: "Size of the cube along each axis",
      },
      {
        name: "color",
        type: { type: "nested", name: "Color" },
        description: "Color of the arrow",
      },
    ],
  },

  "foxglove.Markers.SphereMarker": {
    name: "foxglove.Markers.SphereMarker",
    fields: [
      ...commonMarkerFields,
      {
        name: "pose",
        type: { type: "nested", name: "Pose" },
        description:
          "Position of the center of the sphere and orientation of the sphere",
      },
      {
        name: "size",
        type: { type: "nested", name: "Vector3" },
        description: "Size (diameter) of the sphere along each axis",
      },
      {
        name: "color",
        type: { type: "nested", name: "Color" },
        description: "Color of the sphere",
      },
    ],
  },

  "foxglove.Markers.CylinderMarker": {
    name: "foxglove.Markers.CylinderMarker",
    fields: [
      ...commonMarkerFields,
      {
        name: "pose",
        type: { type: "nested", name: "Pose" },
        description:
          "Position of the center of the cylinder and orientation of the cylinder. The cylinder's flat faces are perpendicular to the z-axis.",
      },
      {
        name: "bottom_radius",
        type: { type: "primitive", name: "float" },
        description: "Radius of the cylinder at min z",
      },
      {
        name: "top_radius",
        type: { type: "primitive", name: "float" },
        description: "Radius of the cylinder at max z",
      },
      {
        name: "height",
        type: { type: "primitive", name: "float" },
        description: "Height of the cylinder along the z axis",
      },
      {
        name: "color",
        type: { type: "nested", name: "Color" },
        description: "Color of the sphere",
      },
    ],
  },

  "foxglove.Markers.LineMarker": {
    name: "foxglove.Markers.LineMarker",
    enums: [
      {
        name: "LineType",
        values: [
          { value: 0, name: "LINE_STRIP", description: "0-1, 1-2, ..." },
          { value: 1, name: "LINE_LOOP", description: "0-1, 1-2, ..., n-0" },
          { value: 1, name: "LINE_LIST", description: "0-1, 2-3, 4-5, ..." },
        ],
      },
    ],
    fields: [
      ...commonMarkerFields,
      {
        name: "pose",
        type: { type: "nested", name: "Pose" },
        description: "Origin of lines relative to reference frame",
      },
      {
        name: "thickness",
        type: { type: "primitive", name: "float" },
        description: "Line thickness",
      },
      {
        name: "scale_invariant",
        type: { type: "primitive", name: "boolean" },
        description:
          "Indicates whether `thickness` is a fixed size in screen pixels (true), or specified in world coordinates and scales with distance from the camera (false)",
      },
      {
        name: "points",
        type: { type: "nested", name: "Point3" },
        array: true,
        description: "Points along the line",
      },
      {
        name: "color",
        type: { type: "nested", name: "Color" },
        description:
          "Solid color to use for the whole line. One of `color` or `colors` must be provided.",
      },
      {
        name: "colors",
        type: { type: "nested", name: "Color" },
        array: true,
        description:
          "Per-point colors (if specified, must have the same length as `points`). One of `color` or `colors` must be provided.",
      },
      {
        name: "indices",
        type: { type: "primitive", name: "integer" },
        array: true,
        description:
          "Indexes into the `points` and `colors` attribute arrays, which can be used to avoid duplicating attribute data.",
      },
    ],
  },

  "foxglove.Markers.TextMarker": {
    name: "foxglove.Markers.TextMarker",
    fields: [
      ...commonMarkerFields,
      {
        name: "pose",
        type: { type: "nested", name: "Pose" },
        description:
          "Position of the center of the text box and orientation of the text. Identity orientation means the text is oriented in the xy-plane and flows from -x to +x.",
      },
      {
        name: "billboard",
        type: { type: "primitive", name: "boolean" },
        description:
          "Whether the text should respect `pose.orientation` (false) or always face the camera (true)",
      },
      {
        name: "font_size",
        type: { type: "primitive", name: "float" },
        description: "Font size (height of one line of text)",
      },
      {
        name: "scale_invariant",
        type: { type: "primitive", name: "boolean" },
        description:
          "Indicates whether `font_size` is a fixed size in screen pixels (true), or specified in world coordinates and scales with distance from the camera (false)",
      },
      {
        name: "color",
        type: { type: "nested", name: "Color" },
        description: "Color of the text",
      },
      {
        name: "text",
        type: { type: "primitive", name: "string" },
        description: "Text",
      },
    ],
  },

  "foxglove.Markers.TrianglesMarker": {
    name: "foxglove.Markers.TrianglesMarker",
    enums: [
      {
        name: "TriangleType",
        values: [
          { value: 0, name: "TRIANGLE_LIST", description: "0-1-2, 3-4-5, ..." },
          {
            value: 1,
            name: "TRIANGLE_STRIP",
            description: "0-1-2, 1-2-3, 2-3-4, ...",
          },
          {
            value: 1,
            name: "TRIANGLE_FAN",
            description: "0-1-2, 0-2-3, 0-3-4, ...",
          },
        ],
      },
    ],
    fields: [
      ...commonMarkerFields,
      {
        name: "pose",
        type: { type: "nested", name: "Pose" },
        description: "Origin of triangles relative to reference frame",
      },
      {
        name: "points",
        type: { type: "nested", name: "Point3" },
        array: true,
        description: "Vertices to use for triangles",
      },
      {
        name: "color",
        type: { type: "nested", name: "Color" },
        description:
          "Solid color to use for the whole shape. One of `color` or `colors` must be provided.",
      },
      {
        name: "colors",
        type: { type: "nested", name: "Color" },
        array: true,
        description:
          "Per-vertex colors (if specified, must have the same length as `points`). One of `color` or `colors` should be provided.",
      },
      {
        name: "indices",
        type: { type: "primitive", name: "integer" },
        array: true,
        description:
          "Indexes into the `points` and `colors` attribute arrays, which can be used to avoid duplicating attribute data.",
      },
    ],
  },

  "foxglove.Markers.ModelMarker": {
    name: "foxglove.Markers.ModelMarker",
    fields: [
      ...commonMarkerFields,
      {
        name: "pose",
        type: { type: "nested", name: "Pose" },
        description: "Origin of model relative to reference frame",
      },
      {
        name: "scale",
        type: { type: "nested", name: "Vector3" },
        description: "Scale factor to apply to the model along each axis",
      },
      {
        name: "color",
        type: { type: "nested", name: "Color" },
        description:
          "Solid color to use for the whole model. If `use_embedded_materials` is true, this color is blended on top of the embedded material color.",
      },
      {
        name: "use_embedded_materials",
        type: { type: "primitive", name: "boolean" },
        description:
          "Whether to use materials embedded in the model, or only the `color`",
      },
      {
        name: "url",
        type: { type: "primitive", name: "string" },
        description:
          "URL pointing to model file. Either `url` or `mime_type` and `data` should be provided.",
      },
      {
        name: "mime_type",
        type: { type: "primitive", name: "string" },
        description:
          "MIME type of embedded model (e.g. `model/gltf-binary`). Either `url` or `mime_type` and `data` should be provided.",
      },
      {
        name: "data",
        type: { type: "primitive", name: "bytes" },
        description:
          "Embedded model. Either `url` or `mime_type` and `data` should be provided.",
      },
    ],
  },
} as const;

const typedFoxgloveSchemas: Record<
  keyof typeof foxgloveSchemas,
  FoxgloveSchema
> = foxgloveSchemas;

export { typedFoxgloveSchemas as foxgloveSchemas };
