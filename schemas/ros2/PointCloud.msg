# Generated from PointCloud by @foxglove/schemas

# Timestamp of point cloud
builtin_interfaces/Time timestamp

# Frame of reference
string frame_id

# The origin of the point cloud relative to the frame of reference
geometry_msgs/Pose pose

# Number of bytes between points in the `data`
uint32 point_stride

# Fields in the `data`
foxglove_msgs/PackedElementField[] fields

# Point data, interpreted using `fields`
uint8[] data
