import { FoxgloveEnumSchema, FoxgloveMessageSchema } from "./types";

const foxglove_Color: FoxgloveMessageSchema = {
  type: "message",
  name: "Color",
  description: "A color in RGBA format",
  fields: [
    {
      name: "r",
      type: { type: "primitive", name: "float64" },
      description: "Red value between 0 and 1",
    },
    {
      name: "g",
      type: { type: "primitive", name: "float64" },
      description: "Green value between 0 and 1",
    },
    {
      name: "b",
      type: { type: "primitive", name: "float64" },
      description: "Blue value between 0 and 1",
    },
    {
      name: "a",
      type: { type: "primitive", name: "float64" },
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
      type: { type: "primitive", name: "float64" },
      description: "x coordinate length",
    },
    {
      name: "y",
      type: { type: "primitive", name: "float64" },
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
      type: { type: "primitive", name: "float64" },
      description: "x coordinate length",
    },
    {
      name: "y",
      type: { type: "primitive", name: "float64" },
      description: "y coordinate length",
    },
    {
      name: "z",
      type: { type: "primitive", name: "float64" },
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
      type: { type: "primitive", name: "float64" },
      description: "x coordinate position",
    },
    {
      name: "y",
      type: { type: "primitive", name: "float64" },
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
      type: { type: "primitive", name: "float64" },
      description: "x coordinate position",
    },
    {
      name: "y",
      type: { type: "primitive", name: "float64" },
      description: "y coordinate position",
    },
    {
      name: "z",
      type: { type: "primitive", name: "float64" },
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
      type: { type: "primitive", name: "float64" },
      description: "x value",
    },
    {
      name: "y",
      type: { type: "primitive", name: "float64" },
      description: "y value",
    },
    {
      name: "z",
      type: { type: "primitive", name: "float64" },
      description: "z value",
    },
    {
      name: "w",
      type: { type: "primitive", name: "float64" },
      description: "w value",
    },
  ],
};

const foxglove_Pose: FoxgloveMessageSchema = {
  type: "message",
  name: "Pose",
  description: "A position and orientation for an object or reference frame in 3D space",
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

const foxglove_CameraCalibration: FoxgloveMessageSchema = {
  type: "message",
  name: "CameraCalibration",
  description: "Camera calibration parameters",
  fields: [
    {
      name: "timestamp",
      type: { type: "primitive", name: "time" },
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
      type: { type: "primitive", name: "float64" },
      description: "Distortion parameters",
      array: true,
    },
    {
      name: "K",
      type: { type: "primitive", name: "float64" },
      array: 9,
      description: `Intrinsic camera matrix (3x3 row-major matrix)

A 3x3 row-major matrix for the raw (distorted) image.

Projects 3D points in the camera coordinate frame to 2D pixel coordinates using the focal lengths (fx, fy) and principal point (cx, cy).

\`\`\`
    [fx  0 cx]
K = [ 0 fy cy]
    [ 0  0  1]
\`\`\`
`,
    },
    {
      name: "R",
      type: { type: "primitive", name: "float64" },
      array: 9,
      description: `Rectification matrix (3x3 row-major matrix)

A rotation matrix aligning the camera coordinate system to the ideal stereo image plane so that epipolar lines in both stereo images are parallel.`,
    },
    {
      name: "P",
      type: { type: "primitive", name: "float64" },
      array: 12,
      description: `Projection/camera matrix (3x4 row-major matrix)

\`\`\`
    [fx'  0  cx' Tx]
P = [ 0  fy' cy' Ty]
    [ 0   0   1   0]
\`\`\`

By convention, this matrix specifies the intrinsic (camera) matrix of the processed (rectified) image. That is, the left 3x3 portion is the normal camera intrinsic matrix for the rectified image.

It projects 3D points in the camera coordinate frame to 2D pixel coordinates using the focal lengths (fx', fy') and principal point (cx', cy') - these may differ from the values in K.

For monocular cameras, Tx = Ty = 0. Normally, monocular cameras will also have R = the identity and P[1:3,1:3] = K.

For a stereo pair, the fourth column [Tx Ty 0]' is related to the position of the optical center of the second camera in the first camera's frame. We assume Tz = 0 so both cameras are in the same stereo image plane. The first camera always has Tx = Ty = 0. For the right (second) camera of a horizontal stereo pair, Ty = 0 and Tx = -fx' * B, where B is the baseline between the cameras.

Given a 3D point [X Y Z]', the projection (x, y) of the point onto the rectified image is given by:

\`\`\`
[u v w]' = P * [X Y Z 1]'
       x = u / w
       y = v / w
\`\`\`

This holds for both images of a stereo pair.
`,
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
      type: { type: "primitive", name: "time" },
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
  description: "A raw image",
  fields: [
    {
      name: "timestamp",
      type: { type: "primitive", name: "time" },
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

const foxglove_FrameTransform: FoxgloveMessageSchema = {
  type: "message",
  name: "FrameTransform",
  description: "A transform between two reference frames in 3D space",
  fields: [
    {
      name: "timestamp",
      type: { type: "primitive", name: "time" },
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

const foxglove_PoseInFrame: FoxgloveMessageSchema = {
  type: "message",
  name: "PoseInFrame",
  description: "A timestamped pose for an object or reference frame in 3D space",
  fields: [
    {
      name: "timestamp",
      type: { type: "primitive", name: "time" },
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
  description: "An array of timestamped poses for an object or reference frame in 3D space",
  fields: [
    {
      name: "timestamp",
      type: { type: "primitive", name: "time" },
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
  description: "GeoJSON data for annotating maps",
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
  protobufParentMessageName: "PackedElementField",
  protobufEnumName: "NumericType",
  values: [
    { name: "UNKNOWN", value: 0 },
    { name: "UINT8", value: 1 },
    { name: "INT8", value: 2 },
    { name: "UINT16", value: 3 },
    { name: "INT16", value: 4 },
    { name: "UINT32", value: 5 },
    { name: "INT32", value: 6 },
    { name: "FLOAT32", value: 7 },
    { name: "FLOAT64", value: 8 },
  ],
};

const foxglove_PackedElementField: FoxgloveMessageSchema = {
  type: "message",
  name: "PackedElementField",
  description: "A field present within each element in a byte array of packed elements.",
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
      description: "Type of data in the field. Integers are stored using little-endian byte order.",
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
      type: { type: "primitive", name: "time" },
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
      type: { type: "nested", schema: foxglove_PackedElementField },
      array: true,
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
  description: "A circle annotation on a 2D image",
  fields: [
    {
      name: "timestamp",
      type: { type: "primitive", name: "time" },
      description: "Timestamp of circle",
    },
    {
      name: "position",
      type: { type: "nested", schema: foxglove_Point2 },
      description: "Center of the circle in 2D image coordinates",
    },
    {
      name: "diameter",
      type: { type: "primitive", name: "float64" },
      description: "Circle diameter",
    },
    {
      name: "thickness",
      type: { type: "primitive", name: "float64" },
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
  protobufParentMessageName: "PointsAnnotation",
  protobufEnumName: "Type",
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
  description: "An array of points on a 2D image",
  fields: [
    {
      name: "timestamp",
      type: { type: "primitive", name: "time" },
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
  protobufParentMessageName: "LocationFix",
  protobufEnumName: "PositionCovarianceType",
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
      type: { type: "primitive", name: "float64" },
      description: "Latitude in degrees",
    },
    {
      name: "longitude",
      type: { type: "primitive", name: "float64" },
      description: "Longitude in degrees",
    },
    {
      name: "altitude",
      type: { type: "primitive", name: "float64" },
      description: "Altitude in meters",
    },
    {
      name: "position_covariance",
      type: { type: "primitive", name: "float64" },
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
  protobufParentMessageName: "Log",
  protobufEnumName: "Level",
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
      type: { type: "primitive", name: "time" },
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
      type: { type: "primitive", name: "time" },
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
      type: { type: "nested", schema: foxglove_PackedElementField },
      array: true,
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
  description: "A single scan from a planar laser range-finder",
  fields: [
    {
      name: "timestamp",
      type: { type: "primitive", name: "time" },
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
      type: { type: "primitive", name: "float64" },
      description: "Bearing of first point, in radians",
    },
    {
      name: "end_angle",
      type: { type: "primitive", name: "float64" },
      description: "Bearing of last point, in radians",
    },
    {
      name: "ranges",
      type: { type: "primitive", name: "float64" },
      description:
        "Distance of detections from origin; assumed to be at equally-spaced angles between `start_angle` and `end_angle`",
      array: true,
    },
    {
      name: "intensities",
      type: { type: "primitive", name: "float64" },
      description: "Intensity of detections",
      array: true,
    },
  ],
};

export const foxgloveMessageSchemas = {
  CameraCalibration: foxglove_CameraCalibration,
  CircleAnnotation: foxglove_CircleAnnotation,
  Color: foxglove_Color,
  CompressedImage: foxglove_CompressedImage,
  PackedElementField: foxglove_PackedElementField,
  FrameTransform: foxglove_FrameTransform,
  GeoJSON: foxglove_GeoJSON,
  Grid: foxglove_Grid,
  ImageAnnotations: foxglove_ImageAnnotations,
  LaserScan: foxglove_LaserScan,
  LocationFix: foxglove_LocationFix,
  Log: foxglove_Log,
  Point2: foxglove_Point2,
  Point3: foxglove_Point3,
  PointCloud: foxglove_PointCloud,
  PointsAnnotation: foxglove_PointsAnnotation,
  Pose: foxglove_Pose,
  PoseInFrame: foxglove_PoseInFrame,
  PosesInFrame: foxglove_PosesInFrame,
  Quaternion: foxglove_Quaternion,
  RawImage: foxglove_RawImage,
  Vector2: foxglove_Vector2,
  Vector3: foxglove_Vector3,
};

export const foxgloveEnumSchemas = {
  LogLevel: foxglove_LogLevel,
  NumericType: foxglove_NumericType,
  PointsAnnotationType: foxglove_PointsAnnotationType,
  PositionCovarianceType: foxglove_PositionCovarianceType,
};
