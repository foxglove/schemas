import unittest

from foxglove.channel import Channel, SchemaDefinition


class TestChannel(unittest.TestCase):
    topic: str

    def setUp(self) -> None:
        self.topic = unittest.TestCase.id(self)

    def test_prohibits_duplicate_topics(self) -> None:
        schema = {"type": "object"}
        _ = Channel("test-duplicate", schema=schema)
        self.assertRaisesRegex(
            ValueError, "already exists", Channel, "test-duplicate", schema=schema
        )

    def test_requires_an_object_schema(self) -> None:
        schema = {"type": "array"}
        self.assertRaisesRegex(
            ValueError,
            "Only object schemas are supported",
            Channel,
            self.topic,
            schema=schema,
        )

    def test_log_dict_on_json_channel(self) -> None:
        channel = Channel(
            self.topic,
            schema={"type": "object", "additionalProperties": True},
        )
        self.assertEqual(channel.message_encoding, "json")

        channel.log({"test": "test"})

    def test_log_must_serialize_on_protobuf_channel(self) -> None:
        schema = SchemaDefinition(
            "my_schema",
            message_encoding="protobuf",
            schema_encoding="protobuf",
            schema_data=b"\x01",
        )
        channel = Channel(self.topic, schema=schema)

        self.assertRaisesRegex(
            ValueError, "Unsupported message type", channel.log, {"test": "test"}
        )
        channel.log(b"\x01")


if __name__ == "__main__":
    unittest.main()
