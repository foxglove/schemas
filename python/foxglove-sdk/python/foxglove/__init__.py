import atexit
from typing import Union
from google.protobuf.message import Message
from ._foxglove_py import (
    record_file,
    enable_log_forwarding,
    disable_log_forwarding,
    start_server,
    shutdown,
)
from .encoding import Encoder, ProtobufEncoder, JsonEncoder
from .channel import Channel

import logging

logging.basicConfig(
    level=logging.DEBUG, format="%(asctime)s [%(levelname)s] %(message)s"
)

atexit.register(shutdown)


def log_level_from_int(level: int) -> str:
    log_levels = {10: "debug", 20: "info", 30: "warn", 40: "error"}
    return log_levels.get(level, "unknown")


def verbose_on(level: Union[int, str] = "debug") -> None:
    if isinstance(level, int):
        assert level in [
            logging.DEBUG,
            logging.INFO,
            logging.WARN,
            logging.ERROR,
        ], ValueError("Invalid log level")
        level = log_level_from_int(level)
    else:
        assert level in ["debug", "info", "warn", "error"], ValueError(
            "Invalid log level"
        )
    logging.debug(f"SDK logging enabled ({level.upper()})")
    enable_log_forwarding(level)


def verbose_off() -> None:
    logging.debug("SDK logging disabled")
    disable_log_forwarding()


def log(topic: str, proto_msg: Message) -> None: ...


__all__ = [
    "Channel",
    "start_server",
    "record_file",
    "Encoder",
    "ProtobufEncoder",
    "JsonEncoder",
    "verbose_on",
    "verbose_off",
    "log",
]
