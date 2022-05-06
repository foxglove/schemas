# Generated from CubeMarker by @foxglove/message-schemas

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

# Position of the center of the cube and orientation of the cube
geometry_msgs/Pose pose

# Size of the cube along each axis
geometry_msgs/Vector3 size

# Color of the arrow
foxglove_msgs/Color color
