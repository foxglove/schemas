import atexit
from typing import Union
from ._foxglove_py import (
    record_file,
    enable_logging,
    disable_logging,
    start_server,
    shutdown,
)


from .encoding import Encoder, JsonEncoder
from .channel import Channel, log

import logging

logging.basicConfig(
    level=logging.DEBUG, format="%(asctime)s [%(levelname)s] %(message)s"
)

atexit.register(shutdown)


def _log_level_from_int(level: int) -> str:
    log_levels = {10: "debug", 20: "info", 30: "warn", 40: "error"}
    return log_levels.get(level, "unknown")


def verbose_on(level: Union[int, str] = "debug") -> None:
    """
    Forward SDK logs to python's logging facility.
    """
    if isinstance(level, int):
        assert level in [
            logging.DEBUG,
            logging.INFO,
            logging.WARN,
            logging.ERROR,
        ], ValueError("Invalid log level")
        level = _log_level_from_int(level)
    else:
        assert level in ["debug", "info", "warn", "error"], ValueError(
            "Invalid log level"
        )
    logging.debug(f"SDK logging enabled ({level.upper()})")
    enable_logging(level)


def verbose_off() -> None:
    """
    Stop forwarding SDK logs
    """
    logging.debug("SDK logging disabled")
    disable_logging()


__all__ = [
    "Channel",
    "start_server",
    "record_file",
    "Encoder",
    "JsonEncoder",
    "verbose_on",
    "verbose_off",
    "log",
]
