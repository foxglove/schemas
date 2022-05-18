# Generated from Grid by @foxglove/schemas

# Timestamp of grid
builtin_interfaces/Time timestamp

# Frame of reference
string frame_id

# Origin of grid's corner relative to frame of reference; grid is positioned in the x-y plane relative to this origin
geometry_msgs/Pose pose

# Number of grid columns
uint32 column_count

# Size of single grid cell along x and y axes, relative to `pose`
foxglove_msgs/Vector2 cell_size

# Number of bytes between rows in `data`
uint32 row_stride

# Number of bytes between cells within a row in `data`
uint32 cell_stride

# Fields in `data`
foxglove_msgs/PackedElementField[] fields

# Grid cell data, interpreted using `fields`, in row-major (y-major) order
uint8[] data
