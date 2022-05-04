# Generated from RawImage by @foxglove/message-schemas

# Timestamp of image
time timestamp

# Image width
int32 width

# Image height
int32 height

# Encoding of the raw image data
string encoding

# Byte length of a single row
int32 step

# Raw image data
uint8[] data
