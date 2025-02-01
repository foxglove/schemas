import json
from base64 import b64encode
from abc import ABC, abstractmethod

from google.protobuf.message import Message as ProtoMessage
from .schemas_utils import build_file_descriptor_set

from typing import Any, Type, Tuple


class Encoder(ABC):
    """Abstract base class for encoders."""

    encoding: str
    schema_encoding: str

    def __init__(self, encoding: str, schema_encoding: str) -> None:
        self.encoding = encoding
        self.schema_encoding = schema_encoding

    @abstractmethod
    def encode(self, msg: Any) -> bytes:
        """Encodes a message into bytes."""
        pass

    @abstractmethod
    def get_schema_info(self, schema: Any) -> Tuple[str, str, bytes]:
        """Returns (schema_name, schema_encoding, schema_str)."""
        pass


class ProtobufEncoder(Encoder):
    """Encoder for Protobuf messages."""

    def __init__(self) -> None:
        super().__init__(encoding="protobuf", schema_encoding="protobuf")

    def encode(self, msg: ProtoMessage) -> bytes:
        if not isinstance(msg, ProtoMessage):
            raise TypeError("Message must be a protobuf message.")
        return msg.SerializeToString()

    def get_schema_info(self, schema: Type[ProtoMessage]) -> Tuple[str, str, bytes]:
        if not issubclass(schema, ProtoMessage):
            raise TypeError("Schema must be a subclass of protobuf Message.")
        schema_name: str = schema.DESCRIPTOR.full_name
        schema_encoding: str = self.schema_encoding
        schema_str: bytes = b64encode(
            build_file_descriptor_set(schema).SerializeToString()
        )
        return schema_name, schema_encoding, schema_str


class JsonEncoder(Encoder):
    """Encoder for JSON messages."""

    def __init__(self) -> None:
        super().__init__(encoding="json", schema_encoding="jsonschema")

    def encode(self, msg: dict) -> bytes:
        if not isinstance(msg, dict):
            raise TypeError("Message must be a dictionary.")
        return json.dumps(msg).encode("utf-8")

    def get_schema_info(self, schema: dict) -> Tuple[str, str, bytes]:
        if not isinstance(schema, dict):
            raise TypeError("Schema must be a dictionary.")
        schema_name: str = "json_msg"
        schema_encoding: str = self.schema_encoding
        schema_str: bytes = json.dumps(schema).encode("utf-8")
        return schema_name, schema_encoding, schema_str
