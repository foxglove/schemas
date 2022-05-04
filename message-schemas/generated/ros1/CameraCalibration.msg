# Generated from CameraCalibration by @foxglove/message-schemas

# Timestamp of calibration data
time timestamp

# Image width
int32 width

# Image height
int32 height

# Name of distortion model
string distortion_model

# Distortion parameters
float64[] D

# Intrinsic camera matrix (3x3 row-major matrix)
float64[9] K

# Rectification matrix (3x3 row-major matrix)
float64[9] R

# Projection/camera matrix (3x4 row-major matrix)
float64[12] P
