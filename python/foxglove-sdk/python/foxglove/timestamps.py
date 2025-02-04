from datetime import datetime, timedelta
from typing import Any, Optional, Protocol, TypeVar, Union

from google.protobuf.timestamp_pb2 import Timestamp as Timestamp_pb2
from google.protobuf.duration_pb2 import Duration as Duration_pb2


class TimeLike(Protocol):
    seconds: Optional[int]
    nanos: Optional[int]


class Timestamp:
    """
    A point in time since the Unix epoch, expressed in seconds and nanoseconds.
    """

    seconds: Optional[int]
    nanos: Optional[int]

    def __init__(
        self,
        *,
        seconds: Optional[int] = None,
        nanos: Optional[int] = None,
    ):
        # todo maybe ts is non-negative? other diffs?
        self.seconds = seconds or 0
        self.nanos = nanos or 0

    def as_pb2(self) -> Timestamp_pb2:
        return Timestamp_pb2(seconds=self.seconds, nanos=self.nanos)


class Duration:
    """
    A signed span of time, expressed in seconds and nanoseconds.
    """

    seconds: Optional[int]
    nanos: Optional[int]

    def __init__(
        self,
        *,
        seconds: Optional[int] = None,
        nanos: Optional[int] = None,
    ):
        self.seconds = seconds or 0
        self.nanos = nanos or 0

    def as_pb2(self) -> Duration_pb2:
        return Duration_pb2(seconds=self.seconds, nanos=self.nanos)


def _normalize_timestamp(stamp: TimeLike) -> None:
    if stamp.nanos is None:
        stamp.nanos = 0

    if stamp.seconds is None:
        stamp.seconds = 0

    secs_from_nanos = int(stamp.nanos // 1e9)
    secs = stamp.seconds + secs_from_nanos

    nanos = int(stamp.nanos % 1e9)

    # todo: deal with negative

    stamp.seconds = secs
    stamp.nanos = nanos


# Timestamp = namedtuple("Timestamp", ["seconds", "nanos"])
# Duration = namedtuple("Duration", ["seconds", "nanos"])

SdkTimestamp = Union[datetime, Timestamp]
SdkDuration = Union[
    timedelta,
    Duration,
]


def _sdk_timestamp_to_pb2(timestamp: SdkTimestamp) -> Timestamp_pb2:
    if isinstance(timestamp, datetime):
        stamp = Timestamp_pb2()
        stamp.FromDatetime(timestamp)
        return stamp
    if isinstance(timestamp, Timestamp):
        _normalize_timestamp(timestamp)
        return timestamp.as_pb2()
    raise TypeError(f"Invalid timestamp: {timestamp}")


def _sdk_duration_to_pb2(duration: SdkDuration) -> Duration_pb2:
    if isinstance(duration, timedelta):
        dur = Duration_pb2()
        dur.FromTimedelta(duration)
        return dur
    if isinstance(duration, Duration):
        _normalize_timestamp(duration)
        return duration.as_pb2()
    raise TypeError(f"Invalid duration: {duration}")
