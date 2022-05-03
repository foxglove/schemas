import { FoxgloveEnumSchema, FoxgloveMessageSchema } from "./types";

const foxglove_Color: FoxgloveMessageSchema = {
  type: "message",
  name: "Color",
  fields: [
    {
      name: "r",
      type: { type: "primitive", name: "float" },
      description: "Red value between 0 and 1",
    },
    {
      name: "g",
      type: { type: "primitive", name: "float" },
      description: "Green value between 0 and 1",
    },
    {
      name: "b",
      type: { type: "primitive", name: "float" },
      description: "Blue value between 0 and 1",
    },
    {
      name: "a",
      type: { type: "primitive", name: "float" },
      description: "Alpha value between 0 and 1",
    },
  ],
};

const foxglove_Vector3: FoxgloveMessageSchema = {
  type: "message",
  name: "Vector3",
  rosEquivalent: "geometry_msgs/Vector3",
  fields: [
    {
      name: "x",
      type: { type: "primitive", name: "float" },
      description: "x coordinate length",
    },
    {
      name: "y",
      type: { type: "primitive", name: "float" },
      description: "y coordinate length",
    },
    {
      name: "z",
      type: { type: "primitive", name: "float" },
      description: "z coordinate length",
    },
  ],
};

const foxglove_Point3: FoxgloveMessageSchema = {
  type: "message",
  name: "Point3",
  rosEquivalent: "geometry_msgs/Point",
  fields: [
    {
      name: "x",
      type: { type: "primitive", name: "float" },
      description: "x coordinate position",
    },
    {
      name: "y",
      type: { type: "primitive", name: "float" },
      description: "y coordinate position",
    },
    {
      name: "z",
      type: { type: "primitive", name: "float" },
      description: "z coordinate position",
    },
  ],
};

const foxglove_Quaternion: FoxgloveMessageSchema = {
  type: "message",
  name: "Quaternion",
  rosEquivalent: "geometry_msgs/Quaternion",
  fields: [
    {
      name: "x",
      type: { type: "primitive", name: "float" },
      description: "x value",
    },
    {
      name: "y",
      type: { type: "primitive", name: "float" },
      description: "y value",
    },
    {
      name: "z",
      type: { type: "primitive", name: "float" },
      description: "z value",
    },
    {
      name: "w",
      type: { type: "primitive", name: "float" },
      description: "w value",
    },
  ],
};

const foxglove_Pose: FoxgloveMessageSchema = {
  type: "message",
  name: "Pose",
  rosEquivalent: "geometry_msgs/Pose",
  fields: [
    {
      name: "position",
      type: { type: "nested", schema: foxglove_Vector3 },
      description: "Point denoting position in 3D space",
    },
    {
      name: "orientation",
      type: { type: "nested", schema: foxglove_Quaternion },
      description: "Quaternion denoting orientation in 3D space",
    },
  ],
};

const foxglove_KeyVauePair: FoxgloveMessageSchema = {
  type: "message",
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
};

/** Fields used in each Marker message */
const commonMarkerFields: FoxgloveMessageSchema["fields"] = [
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
    type: { type: "nested", schema: foxglove_KeyVauePair },
    array: true,
    description:
      "Additional user-provided metadata associated with the marker. Keys must be unique.",
  },
];

const foxglove_MarkerDeletionType: FoxgloveEnumSchema = {
  type: "enum",
  name: "MarkerDeletionType",
  values: [
    { value: 0, name: "MATCHING_NAMESPACE_AND_ID" },
    { value: 1, name: "MATCHING_NAMESPACE" },
    { value: 2, name: "ALL" },
  ],
};

const foxglove_MarkerDeletion: FoxgloveMessageSchema = {
  type: "message",
  name: "MarkerDeletion",
  fields: [
    {
      name: "timestamp",
      type: { type: "primitive", name: "Time" },
      description:
        "Timestamp of the marker. Only matching markers earlier than this timestamp will be deleted.",
    },
    {
      name: "type",
      type: { type: "enum", enum: foxglove_MarkerDeletionType },
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
};

const foxglove_ArrowMarker: FoxgloveMessageSchema = {
  type: "message",
  name: "ArrowMarker",
  fields: [
    ...commonMarkerFields,
    {
      name: "pose",
      type: { type: "nested", schema: foxglove_Pose },
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
      type: { type: "nested", schema: foxglove_Color },
      description: "Color of the arrow",
    },
  ],
};

const foxglove_CubeMarker: FoxgloveMessageSchema = {
  type: "message",
  name: "CubeMarker",
  fields: [
    ...commonMarkerFields,
    {
      name: "pose",
      type: { type: "nested", schema: foxglove_Pose },
      description:
        "Position of the center of the cube and orientation of the cube",
    },
    {
      name: "size",
      type: { type: "nested", schema: foxglove_Vector3 },
      description: "Size of the cube along each axis",
    },
    {
      name: "color",
      type: { type: "nested", schema: foxglove_Color },
      description: "Color of the arrow",
    },
  ],
};

const foxglove_SphereMarker: FoxgloveMessageSchema = {
  type: "message",
  name: "SphereMarker",
  fields: [
    ...commonMarkerFields,
    {
      name: "pose",
      type: { type: "nested", schema: foxglove_Pose },
      description:
        "Position of the center of the sphere and orientation of the sphere",
    },
    {
      name: "size",
      type: { type: "nested", schema: foxglove_Vector3 },
      description: "Size (diameter) of the sphere along each axis",
    },
    {
      name: "color",
      type: { type: "nested", schema: foxglove_Color },
      description: "Color of the sphere",
    },
  ],
};

const foxglove_CylinderMarker: FoxgloveMessageSchema = {
  type: "message",
  name: "CylinderMarker",
  fields: [
    ...commonMarkerFields,
    {
      name: "pose",
      type: { type: "nested", schema: foxglove_Pose },
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
      type: { type: "nested", schema: foxglove_Color },
      description: "Color of the sphere",
    },
  ],
};

const foxglove_LineType: FoxgloveEnumSchema = {
  type: "enum",
  name: "LineType",
  values: [
    { value: 0, name: "LINE_STRIP", description: "0-1, 1-2, ..." },
    { value: 1, name: "LINE_LOOP", description: "0-1, 1-2, ..., n-0" },
    { value: 2, name: "LINE_LIST", description: "0-1, 2-3, 4-5, ..." },
  ],
};

const foxglove_LineMarker: FoxgloveMessageSchema = {
  type: "message",
  name: "LineMarker",
  fields: [
    ...commonMarkerFields,
    {
      name: "type",
      type: { type: "enum", enum: foxglove_LineType },
      description: "Drawing primitive to use for lines",
    },
    {
      name: "pose",
      type: { type: "nested", schema: foxglove_Pose },
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
      type: { type: "nested", schema: foxglove_Point3 },
      array: true,
      description: "Points along the line",
    },
    {
      name: "color",
      type: { type: "nested", schema: foxglove_Color },
      description:
        "Solid color to use for the whole line. One of `color` or `colors` must be provided.",
    },
    {
      name: "colors",
      type: { type: "nested", schema: foxglove_Color },
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
};

const foxglove_TextMarker: FoxgloveMessageSchema = {
  type: "message",
  name: "TextMarker",
  fields: [
    ...commonMarkerFields,
    {
      name: "pose",
      type: { type: "nested", schema: foxglove_Pose },
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
      type: { type: "nested", schema: foxglove_Color },
      description: "Color of the text",
    },
    {
      name: "text",
      type: { type: "primitive", name: "string" },
      description: "Text",
    },
  ],
};

const foxglove_TrianglesType: FoxgloveEnumSchema = {
  type: "enum",
  name: "TriangleType",
  values: [
    { value: 0, name: "TRIANGLE_LIST", description: "0-1-2, 3-4-5, ..." },
    {
      value: 1,
      name: "TRIANGLE_STRIP",
      description: "0-1-2, 1-2-3, 2-3-4, ...",
    },
    {
      value: 2,
      name: "TRIANGLE_FAN",
      description: "0-1-2, 0-2-3, 0-3-4, ...",
    },
  ],
};

const foxglove_TrianglesMarker: FoxgloveMessageSchema = {
  type: "message",
  name: "TrianglesMarker",
  fields: [
    ...commonMarkerFields,
    {
      name: "type",
      type: { type: "enum", enum: foxglove_TrianglesType },
      description: "Drawing primitive to use for triangles",
    },
    {
      name: "pose",
      type: { type: "nested", schema: foxglove_Pose },
      description: "Origin of triangles relative to reference frame",
    },
    {
      name: "points",
      type: { type: "nested", schema: foxglove_Point3 },
      array: true,
      description: "Vertices to use for triangles",
    },
    {
      name: "color",
      type: { type: "nested", schema: foxglove_Color },
      description:
        "Solid color to use for the whole shape. One of `color` or `colors` must be provided.",
    },
    {
      name: "colors",
      type: { type: "nested", schema: foxglove_Color },
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
};

const foxglove_ModelMarker: FoxgloveMessageSchema = {
  type: "message",
  name: "ModelMarker",
  fields: [
    ...commonMarkerFields,
    {
      name: "pose",
      type: { type: "nested", schema: foxglove_Pose },
      description: "Origin of model relative to reference frame",
    },
    {
      name: "scale",
      type: { type: "nested", schema: foxglove_Vector3 },
      description: "Scale factor to apply to the model along each axis",
    },
    {
      name: "color",
      type: { type: "nested", schema: foxglove_Color },
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
};

const foxglove_Markers: FoxgloveMessageSchema = {
  type: "message",
  name: "Markers",
  fields: [
    {
      name: "deletions",
      type: { type: "nested", schema: foxglove_MarkerDeletion },
      array: true,
      description: "Marker deletion actions",
    },
    {
      name: "arrows",
      type: { type: "nested", schema: foxglove_ArrowMarker },
      array: true,
      description: "Arrow markers",
    },
    {
      name: "cubes",
      type: { type: "nested", schema: foxglove_CubeMarker },
      array: true,
      description: "Cube markers",
    },
    {
      name: "spheres",
      type: { type: "nested", schema: foxglove_SphereMarker },
      array: true,
      description: "Sphere markers",
    },
    {
      name: "cylinders",
      type: { type: "nested", schema: foxglove_CylinderMarker },
      array: true,
      description: "Cylinder markers",
    },
    {
      name: "lines",
      type: { type: "nested", schema: foxglove_LineMarker },
      array: true,
      description: "Line markers",
    },
    {
      name: "triangles",
      type: { type: "nested", schema: foxglove_TrianglesMarker },
      array: true,
      description: "Triangles markers",
    },
    {
      name: "texts",
      type: { type: "nested", schema: foxglove_TextMarker },
      array: true,
      description: "Text markers",
    },
    {
      name: "models",
      type: { type: "nested", schema: foxglove_ModelMarker },
      array: true,
      description: "Model markers",
    },
  ],
};

export const foxgloveMessageSchemas = {
  Color: foxglove_Color,
  Pose: foxglove_Pose,
  Vector3: foxglove_Vector3,
  Point3: foxglove_Point3,
  Quaternion: foxglove_Quaternion,
  KeyValuePair: foxglove_KeyVauePair,
  Markers: foxglove_Markers,
  MarkerDeletion: foxglove_MarkerDeletion,
  ArrowMarker: foxglove_ArrowMarker,
  CubeMarker: foxglove_CubeMarker,
  SphereMarker: foxglove_SphereMarker,
  CylinderMarker: foxglove_CylinderMarker,
  LineMarker: foxglove_LineMarker,
  TextMarker: foxglove_TextMarker,
  TrianglesMarker: foxglove_TrianglesMarker,
  ModelMarker: foxglove_ModelMarker,
};

export const foxgloveEnumSchemas = {
  TrianglesType: foxglove_TrianglesType,
  LineType: foxglove_LineType,
  MarkerDeletionType: foxglove_MarkerDeletionType,
};
