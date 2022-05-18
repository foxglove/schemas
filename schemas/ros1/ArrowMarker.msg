# Generated from ArrowMarker by @foxglove/schemas

# Timestamp of the marker
time timestamp

# Frame of reference
string frame_id

# Namespace into which the marker should be grouped. A marker will replace any prior marker on the same topic with the same `namespace` and `id`.
string namespace

# Identifier for the marker. A marker will replace any prior marker on the same topic with the same `namespace` and `id`.
string id

# Length of time (relative to `timestamp`) after which the marker should be automatically removed. Zero value indicates the marker should remain visible until it is replaced or deleted.
duration lifetime

# Whether the marker should keep its location in the fixed frame (false) or follow the frame specified in `frame_id` as it moves relative to the fixed frame (true)
bool frame_locked

# Additional user-provided metadata associated with the marker. Keys must be unique.
foxglove_msgs/KeyValuePair[] metadata

# Position of the arrow's tail and orientation of the arrow. Identity orientation means the arrow points in the +x direction.
geometry_msgs/Pose pose

# Length of the arrow
float64 length

# Diameter of the arrow shaft
float64 shaft_diameter

# Diameter of the arrow head
float64 head_diameter

# Length of the arrow head
float64 head_length

# Color of the arrow
foxglove_msgs/Color color
