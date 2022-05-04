# Generated from CameraCalibration by @foxglove/message-schemas

# Timestamp of calibration data
builtin_interfaces/Time timestamp

# Image width
int32 width

# Image height
int32 height

# Name of distortion model
string distortion_model

# Distortion parameters
float64[] d

# Intrinsic camera matrix (3x3 row-major matrix)
float64[9] k

# Rectification matrix (3x3 row-major matrix)
float64[9] r

# Projection/camera matrix (3x4 row-major matrix)
float64[12] p
