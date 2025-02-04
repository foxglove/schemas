# Test with serialization on the rust side
import cProfile
import math
from os import path
import struct
import tempfile
import time

import foxglove._foxglove_py

# Channels and schemas from the pyo3-generated module
from foxglove._foxglove_py.channels import (
    BasePointCloudChannel,
    BaseSceneUpdateChannel,
    log_point_cloud,
)

from foxglove._foxglove_py.schemas import (
    Duration,
    PackedElementFieldType,
    SceneEntity,
    SceneUpdate,
    Timestamp,
    Vector3,
    Quaternion,
    Color,
    Pose,
    CubePrimitive,
    PointCloud,
    PackedElementField,
)

from foxglove._foxglove_py import record_file


def make_cube_primitive(i: int) -> CubePrimitive:
    return CubePrimitive(
        pose=Pose(
            position=Vector3(x=i % 10, y=0, z=0),
            orientation=Quaternion(x=0, y=0, z=0, w=i),
        ),
        size=Vector3(x=1, y=1, z=1),
        color=Color(r=0, g=1.0, b=0, a=1),
    )


report = "pyo3_wrappers.prof"
mcap_file = "pyo3_wrappers.mcap"
use_temp_dir = True
cube_count = 0
log_count = 1000


def make_point_cloud() -> PointCloud:
    # todo: nested enum class may not be practical with stub gen
    FLOAT32 = PackedElementFieldType.Float32
    UINT32 = PackedElementFieldType.Uint32

    # https://foxglove.dev/blog/visualizing-point-clouds-with-custom-colors
    point_struct = struct.Struct("<fffBBBB")

    t = time.time()
    points = [(x + math.cos(t + y / 5), y, 0) for x in range(100) for y in range(10)]
    buffer = bytearray(point_struct.size * len(points))
    for i, point in enumerate(points):
        x, y, z = point
        r, g, b, a = [120, 120, 120, 255]
        point_struct.pack_into(buffer, i * point_struct.size, x, y, z, b, g, r, a)

    return PointCloud(
        timestamp=Timestamp(seconds=int(time.time()), nanos=0),
        frame_id="box",
        pose=Pose(
            position=Vector3(x=0, y=0, z=0),
            orientation=Quaternion(x=0, y=0, z=0, w=1),
        ),
        point_stride=10,
        fields=[
            PackedElementField(name="x", offset=0, type=FLOAT32),
            PackedElementField(name="y", offset=4, type=FLOAT32),
            PackedElementField(name="z", offset=8, type=FLOAT32),
            PackedElementField(name="rgba", offset=12, type=UINT32),
        ],
        data=bytes(buffer),
    )


def scene_update(channel: BaseSceneUpdateChannel) -> None:
    cubes = [make_cube_primitive(i) for i in range(cube_count)]

    scene_update = SceneUpdate(
        entities=[
            SceneEntity(
                frame_id="box",
                id="box_1",
                timestamp=Timestamp(seconds=int(time.time()), nanos=0),
                lifetime=Duration(seconds=10, nanos=0),
                cubes=cubes,
                frame_locked=False,
            ),
        ],
        deletions=[],
    )

    channel.log(scene_update)


def point_cloud(channel: BasePointCloudChannel) -> None:
    channel.log(make_point_cloud())


def main() -> None:
    dir = tempfile.TemporaryDirectory(prefix="foxglove-sdk-profile")
    filepath = path.join(dir.name, mcap_file) if use_temp_dir else mcap_file
    writer = record_file(filepath)
    box_chan = BaseSceneUpdateChannel(
        topic="/boxes", message_encoding="protobuf", metadata=None
    )
    point_cloud_chan = BasePointCloudChannel(
        topic="/point_cloud", message_encoding="protobuf", metadata=None
    )

    for counter in range(log_count):
        if counter % (log_count / 10) == 0:
            print(".", end="", flush=True)

        scene_update(box_chan)
        point_cloud(point_cloud_chan)

        # log_point_cloud(point_cloud_chan, make_point_cloud(), [1, 2, 3])
        # foxglove._foxglove_py.channels.log_bytes(b"hello")

    print(writer)


if __name__ == "__main__":
    cProfile.run("main()", report, "time")
