# Test with bare google protobuf types.
# We're getting serialization errors ('invalid wire type') here too.

import cProfile

from datetime import datetime
import math
from os import path
import struct
import tempfile
import time


from foxglove.encoding import FoxgloveSchemaEncoder, ProtobufEncoder

from google.protobuf.timestamp_pb2 import Timestamp
from google.protobuf.duration_pb2 import Duration
from foxglove._protobuf import (
    Color,
    CubePrimitive,
    Pose,
    Quaternion,
    SceneEntity,
    SceneUpdate,
    Vector3,
)
import foxglove

from foxglove._foxglove_py import record_file, start_server

output_file = "pb2_native.prof"
mcap_file = "pb2_native.mcap"
use_temp_dir = False
cube_count = 0
log_count = 1


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
                timestamp=Timestamp(seconds=int(time.time()), nanos=0),
                lifetime=Duration(nanos=int(time.time_ns() - int(time.time()) * 1e9)),
                cubes=cubes,
                frame_locked=False,
            ),
        ],
        deletions=[],
    )

    channel.log(scene_update)


def main() -> None:
    dir = tempfile.TemporaryDirectory(prefix="foxglove-sdk-profile")
    filepath = path.join(dir.name, mcap_file) if use_temp_dir else mcap_file
    writer = record_file(filepath)
    server = start_server(host="localhost", port=8765)

    box_chan = foxglove.Channel(
        topic="/boxes", encoder=ProtobufEncoder(), schema=SceneUpdate
    )

    for counter in range(log_count):
        if counter % (log_count / 10) == 0:
            print(".", end="", flush=True)

        while True:
            scene_update(box_chan)
            time.sleep(1)

    print(writer, server)


if __name__ == "__main__":
    cProfile.run("main()", output_file, "time")
