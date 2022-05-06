# Generated from PosesInFrame by @foxglove/message-schemas

# Timestamp of pose
builtin_interfaces/Time timestamp

# Frame of reference for pose position and orientation
string frame_id

# Poses in 3D space
geometry_msgs/Pose[] poses
