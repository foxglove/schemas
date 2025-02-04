from enum import Enum
from typing import List

class Vector3:
    def __new__(cls, x: float, y: float, z: float) -> "Vector3": ...

class Point3:
    def __new__(cls, x: float, y: float, z: float) -> "Point3": ...

class Quaternion:
    def __new__(cls, x: float, y: float, z: float, w: float) -> "Quaternion": ...

class Color:
    def __new__(cls, r: float, g: float, b: float, a: float) -> "Color": ...

class Pose:
    def __new__(cls, position: Vector3, orientation: Quaternion) -> "Pose": ...

class CubePrimitive:
    def __new__(cls, pose: Pose, size: Vector3, color: Color) -> "CubePrimitive": ...

class Timestamp:
    def __new__(cls, seconds: int, nanos: int) -> "Timestamp": ...

class Duration:
    def __new__(cls, seconds: int, nanos: int) -> "Duration": ...

class SceneEntity:
    def __new__(
        cls,
        frame_id: str,
        id: str,
        timestamp: Timestamp,
        lifetime: Duration,
        frame_locked: bool,
        cubes: List[CubePrimitive],
    ) -> "SceneEntity": ...

class SceneEntityDeletion:
    def __new__(cls, id: str) -> "SceneEntityDeletion": ...

class SceneUpdate:
    def __new__(
        cls, entities: List[SceneEntity], deletions: List[SceneEntityDeletion]
    ) -> "SceneUpdate": ...

class PackedElementField:
    def __new__(
        cls, name: str, offset: int, type: "PackedElementFieldType"
    ) -> "PackedElementField": ...

class PackedElementFieldType(Enum):
    Unknown = 0
    Uint8 = 1
    Int8 = 2
    Uint16 = 3
    Int16 = 4
    Uint32 = 5
    Int32 = 6
    Float32 = 7
    Float64 = 8

class PointCloud:
    def __new__(
        cls,
        timestamp: Timestamp,
        frame_id: str,
        pose: Pose,
        point_stride: int,
        fields: List[PackedElementField],
        data: bytes,
    ) -> "PointCloud": ...
