This repo contains Protobuf and JSON schemas generated from the common ROS message types at [@foxglove/rosmsg-msgs-common](https://github.com/foxglove/rosmsg-msgs-common). These schemas can be used to create custom [Foxglove WebSocket protocol](https://github.com/foxglove/ws-protocol) servers that take advantage of Foxglove Studio's support for ROS data visualization without using ROS.

### Generate .proto files and JSON Schemas for common ROS datatypes

```
$ yarn install
$ yarn generate:proto
$ yarn generate:json
```

### Combine .proto files into a binary FileDescriptorSet

```
$ protoc --proto_path=proto proto/ros/sensor_msgs/PointCloud2.proto --include_imports --descriptor_set_out=PointCloud2.bin
```
