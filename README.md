# @foxglove/schemas

> [Message schemas](https://foxglove.dev/docs/studio/messages/introduction) supported by [Foxglove Studio](https://studio.foxglove.dev)

[![npm version](https://img.shields.io/npm/v/@foxglove/schemas.svg?style=flat)](https://www.npmjs.com/package/@foxglove/schemas)

## Introduction

The [schemas](./schemas) folder contains type definitions generated from these schemas for ROS 1, ROS 2, Protobuf, JSON Schema, and TypeScript.

These schemas can be used in [MCAP](https://github.com/foxglove/mcap) files or [Foxglove WebSocket](https://github.com/foxglove/ws-protocol) servers to take advantage of Foxglove Studio's visualizations.

## License

@foxglove/schemas is licensed under the [MIT License](https://opensource.org/licenses/MIT).

## Development

The schema definitions are in [src/schemas.ts](src/schemas.ts).

After editing the schemas, re-generate the language-specific definitions by running `yarn update-generated-files`.

### Releasing to NPM

1. Run `yarn version --[major|minor|patch]` to bump version
2. Run `git push && git push --tags` to push new tag
3. GitHub Actions will take care of the rest

### Releasing foxglove_msgs for ROS

For first-time setup, follow the guides for [installing bloom](http://ros-infrastructure.github.io/bloom/) and [authenticating with GitHub](https://wiki.ros.org/bloom/Tutorials/GithubManualAuthorization).

The following is a modified version of [bloom release instructions](https://wiki.ros.org/bloom/Tutorials/ReleaseCatkinPackage) (because catkin_generate_changelog and catkin_prepare_release can't handle our custom tag format of `ros-vX.Y.Z`).

1. Manually update `package.xml` and `CHANGELOG.rst` with new version info
2. Manually create a tag named `ros-vX.Y.Z` for the new version
3. Push the newly created commit and tag
4. Run `bloom-release foxglove_msgs --ros-distro humble`, for each distro you want to publish the release to. Follow the prompts which will make a PR to the ros/rosdistro repo.

## Stay in touch

Join our [Slack channel](https://foxglove.dev/join-slack) to ask questions, share feedback, and stay up to date on what our team is working on.
