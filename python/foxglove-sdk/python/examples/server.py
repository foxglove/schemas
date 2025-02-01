import foxglove
import numpy as np
import time

from foxglove.schemas import RawImage


json_schema_example = {
    "type": "object",
    "properties": {
        "msg": {"type": "string"},
        "count": {"type": "number"},
    },
}

# You can put these anywhere, likely in a logging.py or channels.py file
EXAMPLE_JSON = foxglove.Channel(
    "/example_json", encoder=foxglove.JsonEncoder(), schema=json_schema_example
)
EXAMPLE_PROTO = foxglove.Channel(
    topic="/example_proto", encoder=foxglove.ProtobufEncoder(), schema=RawImage
)
# For our schemas you could use generated Channel subclasses:
# EXAMPLE_IMAGE = foxglove.RawImageChannel("/example_image")


def main() -> None:
    # We could have a global context if we want
    # ctx = foxglove.default_context()

    # Starts a server (WebSocket) logger
    server = foxglove.start_server("example-server", port=8765)

    try:
        count = 0
        while True:
            count += 1
            json_msg = {
                "msg": "Hello!",
                "count": count,
            }

            # Logging to every initialized logger (ws_server in this case)
            EXAMPLE_JSON.log(json_msg)

            EXAMPLE_PROTO.log(
                RawImage(
                    data=np.zeros((100, 100, 3), dtype=np.uint8).tobytes(),
                    step=300,
                    width=100,
                    height=100,
                    encoding="rgb8",
                )
            )

            # Or use high-level log API without needing to manage explicit Channels.
            # Works with any protobuf message (ours or theirs).
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

            time.sleep(1)
    finally:
        # Clean up
        server.stop()


if __name__ == "__main__":
    try:
        main()
    except KeyboardInterrupt:
        pass
