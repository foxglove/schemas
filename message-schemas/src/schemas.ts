import { FoxgloveEnumSchema, FoxgloveMessageSchema } from "./types";

const foxglove_Color: FoxgloveMessageSchema = {
  type: "message",
  name: "Color",
  description: "A color in RGBA format",
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

const foxglove_Vector2: FoxgloveMessageSchema = {
  type: "message",
  name: "Vector2",
  description: "A vector in 2D space that represents a direction only",
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
  ],
};

const foxglove_Vector3: FoxgloveMessageSchema = {
  type: "message",
  name: "Vector3",
  description: "A vector in 3D space that represents a direction only",
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

const foxglove_Point2: FoxgloveMessageSchema = {
  type: "message",
  name: "Point2",
  description: "A point representing a position in 2D space",
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
  ],
};

const foxglove_Point3: FoxgloveMessageSchema = {
  type: "message",
  name: "Point3",
  description: "A point representing a position in 3D space",
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
  description: "A [quaternion](https://eater.net/quaternions) representing a rotation in 3D space",
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
  description: "The position and orientation of an object or reference frame in 3D space",
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
  description: "An entry representing a key and its associated value",
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
  description: "An enumeration indicating which markers should match a MarkerDeletion command",
  values: [
    { value: 0, name: "MATCHING_NAMESPACE_AND_ID" },
    { value: 1, name: "MATCHING_NAMESPACE" },
    { value: 2, name: "ALL" },
  ],
};

const foxglove_MarkerDeletion: FoxgloveMessageSchema = {
  type: "message",
  name: "MarkerDeletion",
  description: "Deletion command to remove previously published markers",
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
      description: "Numeric identifier which must match if `kind` is `MATCHING_NAMESPACE_AND_ID`.",
    },
  ],
};

const foxglove_ArrowMarker: FoxgloveMessageSchema = {
  type: "message",
  name: "ArrowMarker",
  description: "A marker representing an arrow",
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
  description: "A marker representing a cube or rectangular prism",
  fields: [
    ...commonMarkerFields,
    {
      name: "pose",
      type: { type: "nested", schema: foxglove_Pose },
      description: "Position of the center of the cube and orientation of the cube",
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
  description: "A marker representing a sphere or ellipsoid",
  fields: [
    ...commonMarkerFields,
    {
      name: "pose",
      type: { type: "nested", schema: foxglove_Pose },
      description: "Position of the center of the sphere and orientation of the sphere",
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
  description: "A marker representing a cylinder or elliptic cylinder",
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
  description: "An enumeration indicating how input points should be interpreted to create lines",
  values: [
    { value: 0, name: "LINE_STRIP", description: "0-1, 1-2, ..., (n-1)-n" },
    { value: 1, name: "LINE_LOOP", description: "0-1, 1-2, ..., (n-1)-n, n-0" },
    { value: 2, name: "LINE_LIST", description: "0-1, 2-3, 4-5, ..." },
  ],
};

const foxglove_LineMarker: FoxgloveMessageSchema = {
  type: "message",
  name: "LineMarker",
  description: "A marker representing a series of points connected by lines",
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
      type: { type: "primitive", name: "uint32" },
      array: true,
      description:
        "Indexes into the `points` and `colors` attribute arrays, which can be used to avoid duplicating attribute data.",
    },
  ],
};

const foxglove_TextMarker: FoxgloveMessageSchema = {
  type: "message",
  name: "TextMarker",
  description: "A marker representing a text label",
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
  description:
    "An enumeration indicating how input points should be interpreted to create triangles",
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
  description: "A marker representing a set of triangles or a surface tiled by triangles",
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
      type: { type: "primitive", name: "uint32" },
      array: true,
      description:
        "Indexes into the `points` and `colors` attribute arrays, which can be used to avoid duplicating attribute data.",
    },
  ],
};

const foxglove_ModelMarker: FoxgloveMessageSchema = {
  type: "message",
  name: "ModelMarker",
  description: "A marker representing a 3D model",
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
      description: "Whether to use materials embedded in the model, or only the `color`",
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
      description: "Embedded model. Either `url` or `mime_type` and `data` should be provided.",
    },
  ],
};

const foxglove_Markers: FoxgloveMessageSchema = {
  type: "message",
  name: "Markers",
  description: "A list that can contain any number of any type of marker",
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

export const foxglove_CameraCalibration: FoxgloveMessageSchema = {
  type: "message",
  name: "CameraCalibration",
  description: "Camera calibration parameters",
  fields: [
    {
      name: "timestamp",
      type: { type: "primitive", name: "Time" },
      description: "Timestamp of calibration data",
    },
    {
      name: "width",
      type: { type: "primitive", name: "uint32" },
      description: "Image width",
    },
    {
      name: "height",
      type: { type: "primitive", name: "uint32" },
      description: "Image height",
    },
    {
      name: "distortion_model",
      type: { type: "primitive", name: "string" },
      description: "Name of distortion model",
    },
    {
      name: "D",
      type: { type: "primitive", name: "float" },
      description: "Distortion parameters",
      array: true,
    },
    {
      name: "K",
      type: { type: "primitive", name: "float" },
      description: "Intrinsic camera matrix (3x3 row-major matrix)",
      array: 9,
    },
    {
      name: "R",
      type: { type: "primitive", name: "float" },
      description: "Rectification matrix (3x3 row-major matrix)",
      array: 9,
    },
    {
      name: "P",
      type: { type: "primitive", name: "float" },
      description: "Projection/camera matrix (3x4 row-major matrix)",
      array: 12,
    },
  ],
};

const foxglove_CompressedImage: FoxgloveMessageSchema = {
  type: "message",
  name: "CompressedImage",
  description: "A compressed image",
  fields: [
    {
      name: "timestamp",
      type: { type: "primitive", name: "Time" },
      description: "Timestamp of image",
    },
    {
      name: "data",
      type: { type: "primitive", name: "bytes" },
      description: "Compressed image data",
    },
    {
      name: "format",
      type: { type: "primitive", name: "string" },
      description: "Image format",
    },
  ],
};

const foxglove_RawImage: FoxgloveMessageSchema = {
  type: "message",
  name: "RawImage",
  description: "A compressed image",
  fields: [
    {
      name: "timestamp",
      type: { type: "primitive", name: "Time" },
      description: "Timestamp of image",
    },
    {
      name: "width",
      type: { type: "primitive", name: "uint32" },
      description: "Image width",
    },
    {
      name: "height",
      type: { type: "primitive", name: "uint32" },
      description: "Image height",
    },
    {
      name: "encoding",
      type: { type: "primitive", name: "string" },
      description: "Encoding of the raw image data",
    },
    {
      name: "step",
      type: { type: "primitive", name: "uint32" },
      description: "Byte length of a single row",
    },
    {
      name: "data",
      type: { type: "primitive", name: "bytes" },
      description: "Raw image data",
    },
  ],
};

const foxglove_Transform: FoxgloveMessageSchema = {
  type: "message",
  name: "Transform",
  description: "A transform in 3D space",
  fields: [
    {
      name: "timestamp",
      type: { type: "primitive", name: "Time" },
      description: "Transform time",
    },
    {
      name: "translation",
      type: { type: "nested", schema: foxglove_Vector3 },
      description: "Translation component of the transform",
    },
    {
      name: "rotation",
      type: { type: "nested", schema: foxglove_Quaternion },
      description: "Rotation component of the transform",
    },
  ],
};

const foxglove_FrameTransform: FoxgloveMessageSchema = {
  type: "message",
  name: "FrameTransform",
  description: "A transform between two named coordinate frames in 3D space",
  fields: [
    {
      name: "timestamp",
      type: { type: "primitive", name: "Time" },
      description: "Timestamp of transform",
    },
    {
      name: "parent_frame_id",
      type: { type: "primitive", name: "string" },
      description: "Name of the parent frame",
    },
    {
      name: "child_frame_id",
      type: { type: "primitive", name: "string" },
      description: "Name of the child frame",
    },
    {
      name: "transform",
      type: { type: "nested", schema: foxglove_Transform },
      description: "Transform from parent frame to child frame",
    },
  ],
};

const foxglove_PoseInFrame: FoxgloveMessageSchema = {
  type: "message",
  name: "PoseInFrame",
  description: "A timestamped pose in a named coordinate frame",
  fields: [
    {
      name: "timestamp",
      type: { type: "primitive", name: "Time" },
      description: "Timestamp of pose",
    },
    {
      name: "frame_id",
      type: { type: "primitive", name: "string" },
      description: "Frame of reference for pose position and orientation",
    },
    {
      name: "pose",
      type: { type: "nested", schema: foxglove_Pose },
      description: "Pose in 3D space",
    },
  ],
};

const foxglove_PosesInFrame: FoxgloveMessageSchema = {
  type: "message",
  name: "PosesInFrame",
  description: "An array of timestamped poses in a named coordinate frame",
  fields: [
    {
      name: "timestamp",
      type: { type: "primitive", name: "Time" },
      description: "Timestamp of pose",
    },
    {
      name: "frame_id",
      type: { type: "primitive", name: "string" },
      description: "Frame of reference for pose position and orientation",
    },
    {
      name: "poses",
      type: { type: "nested", schema: foxglove_Pose },
      description: "Poses in 3D space",
      array: true,
    },
  ],
};

const foxglove_GeoJSON: FoxgloveMessageSchema = {
  type: "message",
  name: "GeoJSON",
  description: "GeoJSON data used for annotating maps",
  fields: [
    {
      name: "geojson",
      type: { type: "primitive", name: "string" },
      description: "GeoJSON data encoded as a UTF-8 string",
    },
  ],
};

const foxglove_NumericType: FoxgloveEnumSchema = {
  type: "enum",
  name: "NumericType",
  description: "Numeric type",
  values: [
    { name: "UNKNOWN", value: 0 },
    { name: "UINT8", value: 1 },
    { name: "INT8", value: 2 },
    { name: "UINT16", value: 3, description: "little-endian" },
    { name: "INT16", value: 4, description: "little-endian" },
    { name: "UINT32", value: 5, description: "little-endian" },
    { name: "INT32", value: 6, description: "little-endian" },
    { name: "FLOAT32", value: 7, description: "little-endian" },
    { name: "FLOAT64", value: 8, description: "little-endian" },
  ],
};

const foxglove_DataField: FoxgloveMessageSchema = {
  type: "message",
  name: "DataField",
  description: "List of fields included for every entity in an accompanying `data` field array",
  fields: [
    {
      name: "name",
      type: { type: "primitive", name: "string" },
      description: "Name of the field",
    },
    {
      name: "offset",
      type: { type: "primitive", name: "uint32" },
      description: "Byte offset from start of data buffer",
    },
    {
      name: "type",
      type: { type: "enum", enum: foxglove_NumericType },
      description: "Type of data in field",
    },
  ],
};

const foxglove_Grid: FoxgloveMessageSchema = {
  type: "message",
  name: "Grid",
  description: "A 2D grid of data",
  fields: [
    {
      name: "timestamp",
      type: { type: "primitive", name: "Time" },
      description: "Timestamp of grid",
    },
    {
      name: "frame_id",
      type: { type: "primitive", name: "string" },
      description: "Frame of reference",
    },
    {
      name: "pose",
      type: { type: "nested", schema: foxglove_Pose },
      description:
        "Origin of grid's corner relative to frame of reference; grid is positioned in the x-y plane relative to this origin",
    },
    {
      name: "column_count",
      type: { type: "primitive", name: "uint32" },
      description: "Number of grid columns",
    },
    {
      name: "cell_size",
      type: { type: "nested", schema: foxglove_Vector2 },
      description: "Size of single grid cell along x and y axes, relative to `pose`",
    },
    {
      name: "row_stride",
      type: { type: "primitive", name: "uint32" },
      description: "Number of bytes between rows in `data`",
    },
    {
      name: "cell_stride",
      type: { type: "primitive", name: "uint32" },
      description: "Number of bytes between cells within a row in `data`",
    },
    {
      name: "fields",
      type: { type: "nested", schema: foxglove_DataField },
      description: "Fields in `data`",
    },
    {
      name: "data",
      type: { type: "primitive", name: "bytes" },
      description: "Grid cell data, interpreted using `fields`, in row-major (y-major) order",
    },
  ],
};

const foxglove_CircleAnnotation: FoxgloveMessageSchema = {
  type: "message",
  name: "CircleAnnotation",
  description: "A circle annotation",
  fields: [
    {
      name: "timestamp",
      type: { type: "primitive", name: "Time" },
      description: "Timestamp of circle",
    },
    {
      name: "position",
      type: { type: "nested", schema: foxglove_Point2 },
      description: "Center of the circle in 2D image coordinates",
    },
    {
      name: "diameter",
      type: { type: "primitive", name: "float" },
      description: "Circle diameter",
    },
    {
      name: "thickness",
      type: { type: "primitive", name: "float" },
      description: "Line thickness",
    },
    {
      name: "fill_color",
      type: { type: "nested", schema: foxglove_Color },
      description: "Fill color",
    },
    {
      name: "outline_color",
      type: { type: "nested", schema: foxglove_Color },
      description: "Outline color",
    },
  ],
};

const foxglove_PointsAnnotationType: FoxgloveEnumSchema = {
  type: "enum",
  name: "PointsAnnotationType",
  description: "Type of points annotation",
  values: [
    { name: "UNKNOWN", value: 0 },
    { name: "POINTS", value: 1 },
    { name: "LINE_LOOP", value: 2 },
    { name: "LINE_STRIP", value: 3 },
    { name: "LINE_LIST", value: 4 },
  ],
};

const foxglove_PointsAnnotation: FoxgloveMessageSchema = {
  type: "message",
  name: "PointsAnnotation",
  description: "A set of points",
  fields: [
    {
      name: "timestamp",
      type: { type: "primitive", name: "Time" },
      description: "Timestamp of annotation",
    },
    {
      name: "type",
      type: { type: "enum", enum: foxglove_PointsAnnotationType },
      description: "Type of points annotation to draw",
    },
    {
      name: "points",
      type: { type: "nested", schema: foxglove_Point2 },
      description: "Points in 2D image coordinates",
      array: true,
    },
    {
      name: "outline_colors",
      type: { type: "nested", schema: foxglove_Color },
      description: "Outline colors",
      array: true,
    },
    {
      name: "fill_color",
      type: { type: "nested", schema: foxglove_Color },
      description: "Fill color",
    },
  ],
};

const foxglove_ImageAnnotations: FoxgloveMessageSchema = {
  type: "message",
  name: "ImageAnnotations",
  description: "Array of annotations for a 2D image",
  fields: [
    {
      name: "circles",
      type: { type: "nested", schema: foxglove_CircleAnnotation },
      description: "Circle annotations",
      array: true,
    },
    {
      name: "points",
      type: { type: "nested", schema: foxglove_PointsAnnotation },
      description: "Points annotations",
      array: true,
    },
  ],
};

const foxglove_PositionCovarianceType: FoxgloveEnumSchema = {
  type: "enum",
  name: "PositionCovarianceType",
  description: "Type of position covariance",
  values: [
    { name: "UNKNOWN", value: 0 },
    { name: "APPROXIMATED", value: 1 },
    { name: "DIAGONAL_KNOWN", value: 2 },
    { name: "KNOWN", value: 3 },
  ],
};

const foxglove_LocationFix: FoxgloveMessageSchema = {
  type: "message",
  name: "LocationFix",
  description: "A navigation satellite fix for any Global Navigation Satellite System",
  fields: [
    {
      name: "latitude",
      type: { type: "primitive", name: "float" },
      description: "Latitude in degrees",
    },
    {
      name: "longitude",
      type: { type: "primitive", name: "float" },
      description: "Longitude in degrees",
    },
    {
      name: "altitude",
      type: { type: "primitive", name: "float" },
      description: "Altitude in meters",
    },
    {
      name: "position_covariance",
      type: { type: "primitive", name: "float" },
      description:
        "Position covariance (m^2) defined relative to a tangential plane through the reported position. The components are East, North, and Up (ENU), in row-major order.",
      array: 9,
    },
    {
      name: "position_covariance_type",
      type: { type: "enum", enum: foxglove_PositionCovarianceType },
      description:
        "If `position_covariance` is available, `position_covariance_type` must be set to indicate the type of covariance.",
    },
  ],
};

const foxglove_LogLevel: FoxgloveEnumSchema = {
  type: "enum",
  name: "LogLevel",
  description: "Log level",
  values: [
    { name: "UNKNOWN", value: 0 },
    { name: "DEBUG", value: 1 },
    { name: "INFO", value: 2 },
    { name: "WARNING", value: 3 },
    { name: "ERROR", value: 4 },
    { name: "FATAL", value: 5 },
  ],
};

const foxglove_Log: FoxgloveMessageSchema = {
  type: "message",
  name: "Log",
  description: "A log message",
  fields: [
    {
      name: "timestamp",
      type: { type: "primitive", name: "Time" },
      description: "Timestamp of log message",
    },
    {
      name: "level",
      type: { type: "enum", enum: foxglove_LogLevel },
      description: "Log level",
    },
    {
      name: "message",
      type: { type: "primitive", name: "string" },
      description: "Log message",
    },
    {
      name: "name",
      type: { type: "primitive", name: "string" },
      description: "Process or node name",
    },
    {
      name: "file",
      type: { type: "primitive", name: "string" },
      description: "Filename",
    },
    {
      name: "line",
      type: { type: "primitive", name: "uint32" },
      description: "Line number in the file",
    },
  ],
};

const foxglove_PointCloud: FoxgloveMessageSchema = {
  type: "message",
  name: "PointCloud",
  description:
    "A collection of N-dimensional points, which may contain additional fields with information like normals, intensity, etc.",
  fields: [
    {
      name: "timestamp",
      type: { type: "primitive", name: "Time" },
      description: "Timestamp of point cloud",
    },
    {
      name: "frame_id",
      type: { type: "primitive", name: "string" },
      description: "Frame of reference",
    },
    {
      name: "pose",
      type: { type: "nested", schema: foxglove_Pose },
      description: "The origin of the point cloud relative to the frame of reference",
    },
    {
      name: "point_stride",
      type: { type: "primitive", name: "uint32" },
      description: "Number of bytes between points in the `data`",
    },
    {
      name: "fields",
      type: { type: "nested", schema: foxglove_DataField },
      description: "Fields in the `data`",
    },
    {
      name: "data",
      type: { type: "primitive", name: "bytes" },
      description: "Point data, interpreted using `fields`",
    },
  ],
};

const foxglove_LaserScan: FoxgloveMessageSchema = {
  type: "message",
  name: "LaserScan",
  description: "A a single scan from a planar laser range-finder",
  fields: [
    {
      name: "timestamp",
      type: { type: "primitive", name: "Time" },
      description: "Timestamp of scan",
    },
    {
      name: "frame_id",
      type: { type: "primitive", name: "string" },
      description: "Frame of reference",
    },
    {
      name: "pose",
      type: { type: "nested", schema: foxglove_Pose },
      description:
        "Origin of scan relative to frame of reference; points are positioned in the x-y plane relative to this origin; angles are interpreted as counterclockwise rotations around the z axis with 0 rad being in the +x direction",
    },
    {
      name: "start_angle",
      type: { type: "primitive", name: "float" },
      description: "Bearing of first point, in radians",
    },
    {
      name: "end_angle",
      type: { type: "primitive", name: "float" },
      description: "Bearing of last point, in radians",
    },
    {
      name: "ranges",
      type: { type: "primitive", name: "float" },
      description:
        "Distance of detections from origin; assumed to be at equally-spaced angles between `start_angle` and `end_angle`",
      array: true,
    },
    {
      name: "intensities",
      type: { type: "primitive", name: "float" },
      description: "Intensity of detections",
      array: true,
    },
  ],
};

export const foxgloveMessageSchemas = {
  ArrowMarker: foxglove_ArrowMarker,
  CameraCalibration: foxglove_CameraCalibration,
  CircleAnnotation: foxglove_CircleAnnotation,
  Color: foxglove_Color,
  CompressedImage: foxglove_CompressedImage,
  CubeMarker: foxglove_CubeMarker,
  CylinderMarker: foxglove_CylinderMarker,
  DataField: foxglove_DataField,
  FrameTransform: foxglove_FrameTransform,
  GeoJSON: foxglove_GeoJSON,
  Grid: foxglove_Grid,
  ImageAnnotations: foxglove_ImageAnnotations,
  KeyValuePair: foxglove_KeyVauePair,
  LaserScan: foxglove_LaserScan,
  LineMarker: foxglove_LineMarker,
  LocationFix: foxglove_LocationFix,
  Log: foxglove_Log,
  MarkerDeletion: foxglove_MarkerDeletion,
  Markers: foxglove_Markers,
  ModelMarker: foxglove_ModelMarker,
  Point2: foxglove_Point2,
  Point3: foxglove_Point3,
  PointCloud: foxglove_PointCloud,
  PointsAnnotation: foxglove_PointsAnnotation,
  Pose: foxglove_Pose,
  PoseInFrame: foxglove_PoseInFrame,
  PosesInFrame: foxglove_PosesInFrame,
  Quaternion: foxglove_Quaternion,
  RawImage: foxglove_RawImage,
  SphereMarker: foxglove_SphereMarker,
  TextMarker: foxglove_TextMarker,
  Transform: foxglove_Transform,
  TrianglesMarker: foxglove_TrianglesMarker,
  Vector2: foxglove_Vector2,
  Vector3: foxglove_Vector3,
};

export const foxgloveEnumSchemas = {
  LineType: foxglove_LineType,
  LogLevel: foxglove_LogLevel,
  MarkerDeletionType: foxglove_MarkerDeletionType,
  NumericType: foxglove_NumericType,
  PointsAnnotationType: foxglove_PointsAnnotationType,
  PositionCovarianceType: foxglove_PositionCovarianceType,
  TrianglesType: foxglove_TrianglesType,
};
