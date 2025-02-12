from typing import List, Optional, Tuple

class MCAPWriter:
    def __new__(cls) -> "MCAPWriter": ...
    def close(self) -> None: ...

class WebSocketServer:
    def __new__(cls) -> "WebSocketServer": ...
    def stop(self) -> None: ...
    def clear_session(self, session_id: Optional[str] = None) -> None: ...

class BaseChannel:
    def __new__(
        cls,
        topic: str,
        message_encoding: str,
        schema_name: Optional[str] = None,
        schema_encoding: Optional[str] = None,
        schema_data: Optional[bytes] = None,
        metadata: Optional[List[Tuple[str, str]]] = None,
    ) -> "BaseChannel": ...
    def log(
        self,
        msg: bytes,
        publish_time: Optional[int] = None,
        log_time: Optional[int] = None,
        sequence: Optional[int] = None,
    ) -> None: ...

class PartialMetadata:
    def __new__(
        cls,
        sequence: Optional[int] = None,
        log_time: Optional[int] = None,
        publish_time: Optional[int] = None,
    ) -> "PartialMetadata": ...

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
