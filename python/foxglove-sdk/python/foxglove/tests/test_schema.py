import unittest

from foxglove.schemas import Log


class TestSchema(unittest.TestCase):
    def test_schema_can_read_attr(self) -> None:
        log = Log(message="hello")
        self.assertEqual(log.message, "hello")


if __name__ == "__main__":
    unittest.main()
