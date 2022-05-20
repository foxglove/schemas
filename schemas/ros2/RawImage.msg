# Generated from RawImage by @foxglove/schemas

# Timestamp of image
builtin_interfaces/Time timestamp

# Image width
uint32 width

# Image height
uint32 height

# Encoding of the raw image data (8UC1, 8UC3, 16UC1, 32FC1, bayer_bggr8, bayer_gbrg8, bayer_grbg8, bayer_rggb8, bgr8, mono8, mono16, rgb8, or yuv422)
string encoding

# Byte length of a single row
uint32 step

# Raw image data
uint8[] data
