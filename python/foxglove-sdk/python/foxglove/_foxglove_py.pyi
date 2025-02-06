from typing import List, Optional, Tuple

class MCAPWriter:
    def __init__(self) -> None: ...

class WebSocketServer:
    def __init__(self) -> None: ...
    def stop(self) -> None: ...

class BaseChannel:
    def __init__(
        self,
        topic: str,
        message_encoding: str,
        schema_name: Optional[str] = None,
        schema_encoding: Optional[str] = None,
        schema_data: Optional[bytes] = None,
        metadata: Optional[List[Tuple[str, str]]] = None,
    ) -> None: ...
    def log(
        self,
        msg: bytes,
        publish_time: Optional[int] = None,
        log_time: Optional[int] = None,
        sequence: Optional[int] = None,
    ) -> None: ...

def start_server(
    name: Optional[str] = None,
    host: Optional[str] = "127.0.0.1",
    port: Optional[int] = 0,
) -> WebSocketServer: ...
def enable_log_forwarding(level: str) -> None: ...
def disable_log_forwarding() -> None: ...
def shutdown() -> None: ...
def record_file(path: str) -> None: ...
def get_channel_for_topic(topic: str) -> BaseChannel: ...
