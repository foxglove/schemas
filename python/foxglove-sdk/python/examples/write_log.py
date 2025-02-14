import argparse
import inspect
import foxglove

from foxglove.channels import LogChannel
from foxglove.schemas import Log, LogLevel

parser = argparse.ArgumentParser()
parser.add_argument("--path", type=str, default="output.mcap")
args = parser.parse_args()


def main():
    # Create a new mcap file at the given path for recording
    writer = foxglove.record_file(args.path)

    channel = LogChannel("/hello")

    for i in range(10):
        frame = inspect.currentframe()
        frameinfo = inspect.getframeinfo(frame) if frame else None

        channel.log(
            Log(
                level=LogLevel.Info,
                name="SDK example",
                file=frameinfo.filename if frameinfo else None,
                line=frameinfo.lineno if frameinfo else None,
                message=f"message {i}",
            )
        )

    writer.close()


if __name__ == "__main__":
    main()
