import json
from abc import ABC, abstractmethod

from typing import Any, Tuple


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
