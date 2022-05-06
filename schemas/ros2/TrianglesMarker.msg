# Generated from TrianglesMarker by @foxglove/message-schemas

# Timestamp of the marker
builtin_interfaces/Time timestamp

# Frame of reference
string frame_id

# Namespace into which the marker should be grouped. A marker will replace any prior marker on the same topic with the same `namespace` and `id`.
string namespace

# Identifier for the marker. A marker will replace any prior marker on the same topic with the same `namespace` and `id`.
string id

# Length of time (relative to `timestamp`) after which the marker should be automatically removed. Zero value indicates the marker should remain visible until it is replaced or deleted.
builtin_interfaces/Duration lifetime

# Whether the marker should keep its location in the fixed frame (false) or follow the frame specified in `frame_id` as it moves relative to the fixed frame (true)
bool frame_locked

# Additional user-provided metadata associated with the marker. Keys must be unique.
foxglove_msgs/KeyValuePair[] metadata

# 0-1-2, 3-4-5, ...
uint8 TRIANGLE_LIST=0

# 0-1-2, 1-2-3, 2-3-4, ...
uint8 TRIANGLE_STRIP=1

# 0-1-2, 0-2-3, 0-3-4, ...
uint8 TRIANGLE_FAN=2

# Drawing primitive to use for triangles
uint8 type

# Origin of triangles relative to reference frame
geometry_msgs/Pose pose

# Vertices to use for triangles
geometry_msgs/Point[] points

# Solid color to use for the whole shape. One of `color` or `colors` must be provided.
foxglove_msgs/Color color

# Per-vertex colors (if specified, must have the same length as `points`). One of `color` or `colors` should be provided.
foxglove_msgs/Color[] colors

# Indexes into the `points` and `colors` attribute arrays, which can be used to avoid duplicating attribute data.
uint32[] indices
