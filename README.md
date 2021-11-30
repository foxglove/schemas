[WIP]

### Generate .proto files for common ROS datatypes

```
$ yarn install
$ yarn generate
```

### Combine .proto files into a binary FileDescriptorSet

```
$ protoc --proto_path=proto proto/ros/sensor_msgs/PointCloud2.proto --include_imports --descriptor_set_out=PointCloud2.bin
```
