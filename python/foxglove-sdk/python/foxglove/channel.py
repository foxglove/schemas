from ._foxglove_py import BaseChannel
from .encoding import Encoder, ProtobufEncoder
from google.protobuf import message as _message

from typing import Any, Dict, Optional


class Channel:
    __slots__ = ["base", "encoder"]
    base: BaseChannel
    encoder: Encoder

    def __init__(self, topic: str, encoder: Encoder, schema: Any):
        if topic in _channels_by_topic:
            raise ValueError(f"Channel for topic '{topic}' already exists")

        schema_name, schema_encoding, schema_data = encoder.get_schema_info(schema)
        self.base = BaseChannel(
            topic, encoder.encoding, schema_name, schema_encoding, schema_data
        )
        self.encoder = encoder
        _channels_by_topic[topic] = self

    def log(self, msg: Any) -> None:
        payload = self.encoder.encode(msg)
        self.base.log(payload)


_channels_by_topic: Dict[str, Channel] = {}


def log(topic: str, proto_msg: _message.Message) -> None:
    channel: Optional[Channel] = _channels_by_topic.get(topic, None)
    if channel is None:
        channel = Channel(topic, ProtobufEncoder(), proto_msg)
    else:
        # TODO: Check schema compatibility with proto_msg
        pass

    channel.log(proto_msg)
