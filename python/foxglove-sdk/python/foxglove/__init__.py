import atexit
from contextlib import contextmanager
from typing import Iterator, Union
from ._foxglove_py import (
    record_file,
    enable_logging,
    disable_logging,
    start_server,
    shutdown,
    Capability,
)


from .channel import Channel, log, SchemaDefinition

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
    Enable SDK logging.
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
    Disable SDK logging.
    """
    logging.debug("SDK logging disabled")
    disable_logging()


@contextmanager
def new_mcap_file(fname: str) -> Iterator[None]:
    """
    Create an MCAP file at the given path for recording.
    This is the context-managed equivalent of `record_file`.
    """
    writer = record_file(fname)
    try:
        yield
    finally:
        writer.close()


__all__ = [
    "Capability",
    "Channel",
    "start_server",
    "record_file",
    "verbose_on",
    "verbose_off",
    "log",
    "SchemaDefinition",
]
