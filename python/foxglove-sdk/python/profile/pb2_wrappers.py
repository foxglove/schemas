# Test with serialization on the python side, but with a wrapping class for each schema
# to better control the API.
import cProfile

from datetime import datetime
import math
from os import path
import struct
import tempfile
import time


from foxglove.encoding import FoxgloveSchemaEncoder

from foxglove.pb2_wrappers import (
    Color,
    CubePrimitive,
    PackedElementField,
    Point3,
    PointCloud,
    Pose,
    Quaternion,
    SceneEntity,
    SceneUpdate,
    Vector3,
)
import foxglove

from foxglove._foxglove_py import record_file

from foxglove.timestamps import Duration as SdkDuration

output_file = "pb2_wrappers.prof"
mcap_file = "pb2_wrappers.mcap"
use_temp_dir = False
cube_count = 0
log_count = 1000


# todo: numpy support would be nice
def make_point_cloud() -> PointCloud:
    FLOAT32 = PackedElementField.NumericType.FLOAT32
    UINT32 = PackedElementField.NumericType.UINT32

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


def make_cube_primitive(i: int) -> CubePrimitive:
    return CubePrimitive(
        pose=Pose(
            position=Vector3(x=i % 10, y=0, z=0),
            orientation=Quaternion(x=0, y=0, z=0, w=i),
        ),
        size=Vector3(x=1, y=1, z=1),
        color=Color(r=0, g=1.0, b=0, a=1),
    )


def scene_update(channel: foxglove.Channel) -> None:
    # todo: error in this output.
    cubes = [make_cube_primitive(i) for i in range(cube_count)]

    scene_update = SceneUpdate(
        entities=[
            SceneEntity(
                frame_id="box",
                id="box_1",
                timestamp=datetime.now(),
                lifetime=SdkDuration(nanos=time.time_ns()),
                cubes=cubes,
                frame_locked=False,
            ),
        ],
        deletions=[],
    )

    channel.log(scene_update)


def point_cloud(channel: foxglove.Channel) -> None:
    channel.log(make_point_cloud())


def main() -> None:
    dir = tempfile.TemporaryDirectory(prefix="foxglove-sdk-profile")
    filepath = path.join(dir.name, mcap_file) if use_temp_dir else mcap_file
    writer = record_file(filepath)
    # box_chan = foxglove.Channel(
    #     topic="/boxes", encoder=FoxgloveSchemaEncoder(), schema=SceneUpdate
    # )

    point_cloud_chan = foxglove.Channel(
        topic="/point_cloud", encoder=FoxgloveSchemaEncoder(), schema=PointCloud
    )

    for counter in range(log_count):
        if counter % (log_count / 10) == 0:
            print(".", end="", flush=True)

        #scene_update(box_chan)
        point_cloud(point_cloud_chan)

    print(writer)


if __name__ == "__main__":
    cProfile.run("main()", output_file, "time")
