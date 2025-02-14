import json
import foxglove
import numpy as np
import time

from examples.geometry import euler_to_quaternion

from foxglove import SchemaDefinition
from foxglove.channels import SceneUpdateChannel, FrameTransformChannel
from foxglove.schemas import (
    Color,
    CubePrimitive,
    Duration,
    FrameTransform,
    Pose,
    RawImage,
    SceneEntity,
    SceneUpdate,
    Vector3,
)

any_schema = {
    "type": "object",
    "additionalProperties": True,
}

plot_schema = {
    "type": "object",
    "properties": {
        "timestamp": {"type": "number"},
        "y": {"type": "number"},
    },
}


def main() -> None:
    foxglove.verbose_on()

    server = foxglove.start_server(port=8765)

    # Log messages having well-known Foxglove schemas using the appropriate channel type.
    box_chan = SceneUpdateChannel("/boxes")
    tf_chan = FrameTransformChannel("/tf")

    # Log dicts using JSON encoding
    json_chan = foxglove.Channel(topic="/json", schema=plot_schema)

    # Log messages with a custom schema and any encoding
    sin_chan = foxglove.Channel(
        topic="/sine",
        schema=SchemaDefinition(
            name="sine",
            schema_encoding="jsonschema",
            message_encoding="json",
            schema_data=json.dumps(plot_schema).encode("utf-8"),
        ),
    )

    try:
        counter = 0
        while True:
            counter += 1
            now = time.time()
            y = np.sin(now)

            json_msg = {
                "timestamp": now,
                "y": y,
            }
            sin_chan.log(json.dumps(json_msg).encode("utf-8"))

            json_chan.log(json_msg)

            tf_chan.log(
                FrameTransform(
                    parent_frame_id="world",
                    child_frame_id="box",
                    rotation=euler_to_quaternion(roll=1, pitch=0, yaw=counter * 0.1),
                )
            )

            box_chan.log(
                SceneUpdate(
                    entities=[
                        SceneEntity(
                            frame_id="box",
                            id="box_1",
                            lifetime=Duration(seconds=10),
                            cubes=[
                                CubePrimitive(
                                    pose=Pose(
                                        position=Vector3(x=0, y=y, z=3),
                                        orientation=euler_to_quaternion(
                                            roll=0, pitch=0, yaw=counter * -0.1
                                        ),
                                    ),
                                    size=Vector3(x=1, y=1, z=1),
                                    color=Color(r=1.0, g=0, b=0, a=1),
                                )
                            ],
                        ),
                    ]
                )
            )

            # Or use high-level log API without needing to manage explicit Channels.
            foxglove.log(
                "/high-level",
                RawImage(
                    data=np.zeros((100, 100, 3), dtype=np.uint8).tobytes(),
                    step=300,
                    width=100,
                    height=100,
                    encoding="rgb8",
                ),
            )

            time.sleep(0.05)

    except KeyboardInterrupt:
        server.stop()


if __name__ == "__main__":
    main()
