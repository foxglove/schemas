# Foxglove SDK

This repo provides libraries and schemas to log and visualize multimodal data with [Foxglove](https://foxglove.dev).

Visit [Foxglove Docs](https://docs.foxglove.dev/) to get started.

## Packages

| Package                     | Version                                                                                                                         | Description                                                                                                                                                                 |
| --------------------------- | ------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Python**                  |                                                                                                                                 |                                                                                                                                                                             |
| foxglove-schemas-flatbuffer | [![pypi version](https://shields.io/pypi/v/foxglove-schemas-flatbuffer)](https://pypi.org/project/foxglove-schemas-flatbuffer/) | Compiled flatbuffers                                                                                                                                                        |
| foxglove-schemas-protobuf   | [![pypi version](https://shields.io/pypi/v/foxglove-schemas-protobuf)](https://pypi.org/project/foxglove-schemas-protobuf/)     | Compiled protocol buffers                                                                                                                                                   |
| foxglove-websocket          | [![pypi version](https://shields.io/pypi/v/foxglove-websocket)](https://pypi.org/project/foxglove-websocket/)                   | Python implementation of the WebSocket protocol<br/>Repo: [foxglove/ws-protocol](https://github.com/foxglove/ws-protocol/tree/main/python)                                  |
| **C++**                     |                                                                                                                                 |                                                                                                                                                                             |
| foxglove-websocket          | [![conan version](https://img.shields.io/conan/v/foxglove-websocket)](https://conan.io/center/recipes/foxglove-websocket)       | C++ implementation of the WebSocket protocol<br/>Repo: [foxglove/ws-protocol](https://github.com/foxglove/ws-protocol/tree/main/cpp)                                        |
| **ROS**                     |                                                                                                                                 |                                                                                                                                                                             |
| foxglove_msgs               | [![ros version](https://img.shields.io/ros/v/rolling/foxglove_msgs)](https://index.ros.org/p/foxglove_msgs/)                    | Foxglove message definitions for ROS 1 and ROS 2                                                                                                                            |
| **TypeScript**              |                                                                                                                                 |                                                                                                                                                                             |
| @foxglove/schemas           | [![npm version](https://img.shields.io/npm/v/@foxglove/schemas)](https://www.npmjs.com/package/@foxglove/schemas)               | Foxglove schemas for JavaScript / TypeScript                                                                                                                                |
| @foxglove/ws-protocol       | [![npm version](https://img.shields.io/npm/v/@foxglove/ws-protocol)](https://www.npmjs.com/package/@foxglove/ws-protocol)       | JavaScript / TypeScript implementation of the WebSocket protocol<br/>Repo: [foxglove/ws-protocol](https://github.com/foxglove/ws-protocol/tree/main/typescript/ws-protocol) |

### Other

The [schemas](./schemas) directory contains type definitions for ROS 1, ROS 2, Protobuf, JSON Schema, TypeScript, and OMG IDL. They can be copied and used in your application directly.

## Stay in touch

Join our [Discord community](https://foxglove.dev/chat) to ask questions, share feedback, and stay up to date on what our team is working on.
