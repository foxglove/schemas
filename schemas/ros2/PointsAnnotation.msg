# Generated from PointsAnnotation by @foxglove/schemas

# Timestamp of annotation
builtin_interfaces/Time timestamp

uint8 UNKNOWN=0
uint8 POINTS=1
uint8 LINE_LOOP=2
uint8 LINE_STRIP=3
uint8 LINE_LIST=4

# Type of points annotation to draw
uint8 type

# Points in 2D image coordinates
foxglove_msgs/Point2[] points

# Outline colors
foxglove_msgs/Color[] outline_colors

# Fill color
foxglove_msgs/Color fill_color
