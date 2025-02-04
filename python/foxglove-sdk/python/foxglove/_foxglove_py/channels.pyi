from typing import List, Optional, Tuple
from .schemas import SceneUpdate, PointCloud

class BaseSceneUpdateChannel:
    def __init__(
        self,
        topic: str,
        message_encoding: str,
        metadata: Optional[List[Tuple[str, str]]] = None,
    ) -> None: ...
    def log(
        self,
        msg: "SceneUpdate",
        publish_time: Optional[int] = None,
        log_time: Optional[int] = None,
        sequence: Optional[int] = None,
    ) -> None: ...

class BasePointCloudChannel:
    def __init__(
        self,
        topic: str,
        message_encoding: str,
        metadata: Optional[List[Tuple[str, str]]] = None,
    ) -> None: ...
    def log(
        self,
        msg: "PointCloud",
        publish_time: Optional[int] = None,
        log_time: Optional[int] = None,
        sequence: Optional[int] = None,
    ) -> None: ...
