This repo contains Protobuf and JSON schemas for [foxglove](https://foxglove.dev/docs/studio/messages/introduction) and ROS messages.

These schemas can be used in [MCAP](https://github.com/foxglove/mcap) files or [Foxglove WebSocket](https://github.com/foxglove/ws-protocol) servers to take advantage of Foxglove Studio's visualizations.

The ROS schemas are generated from the common ROS message types at [@foxglove/rosmsg-msgs-common](https://github.com/foxglove/rosmsg-msgs-common) and [@foxglove/rosmsg-msgs-foxglove](https://github.com/foxglove/rosmsg-msgs-foxglove).

## Proto

The .proto schemas are located in the `proto` folder.

## Generate

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
