from ._foxglove_py import BaseChannel, channels
from .encoding import Encoder

from typing import Any, Dict, Optional


class Channel:
    __slots__ = ["base", "encoder"]
    base: BaseChannel
    encoder: Optional[Encoder]

    def __init__(self, topic: str, *, schema: Any, encoder: Optional[Encoder] = None):
        if topic in _channels_by_topic:
            raise ValueError(f"Channel for topic '{topic}' already exists")

        self.encoder = encoder

        if encoder is not None:
            schema_name, schema_encoding, schema_data = encoder.get_schema_info(schema)
            self.base = BaseChannel(
                topic, encoder.encoding, schema_name, schema_encoding, schema_data
            )
        else:
            self.base = BaseChannel(topic, "", None, None, None)

        _channels_by_topic[topic] = self

    def log(self, msg: Any) -> None:
        if self.encoder is not None:
            payload = self.encoder.encode(msg)
        else:
            payload = msg

        self.base.log(payload)


_channels_by_topic: Dict[str, Channel] = {}


def log(topic: str, message: object) -> None:
    channel: Optional[Channel] = _channels_by_topic.get(topic, None)
    if channel is None:
        schema_name = type(message).__name__
        channel_name = f"{schema_name}Channel"
        channel_cls = getattr(channels, channel_name, None)
        if channel_cls is not None:
            channel = channel_cls(topic)
        if channel is None:
            raise ValueError(
                f"No Foxglove schema channel found for message type {type(message).__name__}"
            )
        _channels_by_topic[topic] = channel
    else:
        # TODO: Check schema compatibility with proto_msg
        pass

    channel.log(message)
