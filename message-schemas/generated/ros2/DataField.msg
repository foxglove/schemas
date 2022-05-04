# Generated from DataField by @foxglove/message-schemas

# Name of the field
string name

# Byte offset from start of data buffer
int32 offset

uint8 UINT8=1
uint8 INT8=2

# little-endian
uint8 UINT16=3

# little-endian
uint8 INT16=4

# little-endian
uint8 UINT32=5

# little-endian
uint8 INT32=6

# little-endian
uint8 FLOAT32=7

# little-endian
uint8 FLOAT64=8

# Type of data in field
uint8 type
