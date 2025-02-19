# MCAP playback

An example from the Foxglove SDK.

This example reads the given MCAP file and streams the data to a Foxglove WebSocket server, using the
"time" capability of the live visualization server to sync playback with the file's log time.

## Usage

This example uses Poetry: https://python-poetry.org/

```bash
poetry install
poetry run python main.py --file <path-to-mcap-file>
```
