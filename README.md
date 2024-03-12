# @foxglove/schemas

Message schemas supported by [Foxglove](https://foxglove.dev)

| Language/Framework    | Package name                  | Version                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| --------------------- | ----------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| JavaScript/TypeScript | `@foxglove/schemas`           | [![npm version](https://img.shields.io/npm/v/@foxglove/schemas)](https://www.npmjs.com/package/@foxglove/schemas)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| Python + Protobuf     | `foxglove-schemas-protobuf`   | [![pypi version](https://shields.io/pypi/v/foxglove-schemas-protobuf)](https://pypi.org/project/foxglove-schemas-protobuf/)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| Python + FlatBuffers  | `foxglove-schemas-flatbuffer` | [![pypi version](https://shields.io/pypi/v/foxglove-schemas-flatbuffer)](https://pypi.org/project/foxglove-schemas-flatbuffer/)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| ROS                   | `foxglove_msgs`               | [![ROS Melodic version](https://img.shields.io/ros/v/melodic/foxglove_msgs)](https://index.ros.org/p/foxglove_msgs/github-foxglove-schemas/#melodic) [![ROS Noetic version](https://img.shields.io/ros/v/noetic/foxglove_msgs)](https://index.ros.org/p/foxglove_msgs/github-foxglove-schemas/#noetic) [![ROS Foxy version](https://img.shields.io/ros/v/foxy/foxglove_msgs)](https://index.ros.org/p/foxglove_msgs/github-foxglove-schemas/#foxy) [![ROS Galactic version](https://img.shields.io/ros/v/galactic/foxglove_msgs)](https://index.ros.org/p/foxglove_msgs/github-foxglove-schemas/#galactic) [![ROS Humble version](https://img.shields.io/ros/v/humble/foxglove_msgs)](https://index.ros.org/p/foxglove_msgs/github-foxglove-schemas/#humble) [![ROS Iron version](https://img.shields.io/ros/v/iron/foxglove_msgs)](https://index.ros.org/p/foxglove_msgs/github-foxglove-schemas/#iron) [![ROS Rolling version](https://img.shields.io/ros/v/rolling/foxglove_msgs)](https://index.ros.org/p/foxglove_msgs/github-foxglove-schemas/#rolling) |

## Introduction

See [Foxglove Schemas documentation](https://docs.foxglove.dev/docs/visualization/message-schemas/introduction).

The [schemas](./schemas) folder contains type definitions generated for ROS 1, ROS 2, Protobuf, JSON Schema, TypeScript, and OMG IDL.

These schemas can be used in [MCAP](https://github.com/foxglove/mcap) files or [Foxglove WebSocket](https://github.com/foxglove/ws-protocol) servers to take advantage of Foxglove's visualizations.

## License

@foxglove/schemas is licensed under the [MIT License](https://opensource.org/licenses/MIT).

## Development

The schema definitions are in [internal/schemas.ts](internal/schemas.ts).

After editing the schemas, re-generate the language-specific definitions by running `yarn update-generated-files`.

### Release instructions

#### TypeScript

1. Update the version number in `package.json`, make a PR, and merge to main
2. Make a git tag of the form `releases/typescript/vX.Y.Z` on the squash-merged commit, and push the tag
3. GitHub Actions will take care of the rest

#### Python

1. Update the version number in `python/[package-name]/setup.cfg`, make a PR, and merge to main
2. Make a git tag of the form `releases/python/[package-name]/vX.Y.Z` on the squash-merged commit, and push the tag
3. GitHub Actions will take care of the rest

#### ROS

For first-time setup, follow the guides for [installing bloom](http://ros-infrastructure.github.io/bloom/) and [authenticating with GitHub](https://wiki.ros.org/bloom/Tutorials/GithubManualAuthorization).

Permissions to push to [foxglove/ros_foxglove_msgs-release](https://github.com/foxglove/ros_foxglove_msgs-release) (for ROS 1) and [ros2-gbp/ros_foxglove_msgs-release](https://github.com/ros2-gbp/ros_foxglove_msgs-release) (for ROS 2) are required. The latter are managed [via Terraform](https://github.com/ros2-gbp/ros2-gbp-github-org/blob/latest/foxglove_msgs.tf).

The following is a modified version of [bloom release instructions](https://wiki.ros.org/bloom/Tutorials/ReleaseCatkinPackage) (because catkin_generate_changelog and catkin_prepare_release can't handle our custom tag format of `ros-vX.Y.Z`).

1. Manually update `package.xml` and `CHANGELOG.rst` with new version info
2. Manually create a tag named `ros-vX.Y.Z` for the new version
3. Push the newly created commit and tag
4. Run `bloom-release foxglove_msgs --ros-distro humble`, for each distro you want to publish the release to. Follow the prompts, and the script will automatically make a PR to the [ros/rosdistro](https://github.com/ros/rosdistro) repo.

Packages will be available via apt after the [next sync](https://discourse.ros.org/c/release/16). View package build status prior to the sync at: [melodic](http://repositories.ros.org/status_page/ros_melodic_default.html?q=foxglove), [noetic](http://repositories.ros.org/status_page/ros_noetic_default.html?q=foxglove), [foxy](http://repo.ros2.org/status_page/ros_foxy_default.html?q=foxglove), [galactic](http://repo.ros2.org/status_page/ros_galactic_default.html?q=foxglove), [humble](http://repo.ros2.org/status_page/ros_humble_default.html?q=foxglove), [iron](http://repo.ros2.org/status_page/ros_iron_default.html?q=foxglove), [rolling](http://repo.ros2.org/status_page/ros_rolling_default.html?q=foxglove)

## Stay in touch

Join our [Slack channel](https://foxglove.dev/slack) to ask questions, share feedback, and stay up to date on what our team is working on.
