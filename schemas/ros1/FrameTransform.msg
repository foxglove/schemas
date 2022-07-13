# Generated from FrameTransform by @foxglove/schemas

# Timestamp of transform
time timestamp

# Name of the parent frame
string parent_frame_id

# Name of the child frame
string child_frame_id

# Translation component of the transform
geometry_msgs/Vector3 translation

# Rotation component of the transform
geometry_msgs/Quaternion rotation
