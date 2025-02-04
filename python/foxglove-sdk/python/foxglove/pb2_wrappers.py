# Quick & dirty wrapping classes around google protobuf types
# as a comparison against serializing python messages on the rust side

from abc import ABC
from collections import namedtuple
from datetime import datetime, timedelta
from enum import Enum
from typing import Any, Optional, Protocol, TypeVar, Union

from google.protobuf.timestamp_pb2 import Timestamp as Timestamp_pb2
from google.protobuf.duration_pb2 import Duration as Duration_pb2


from foxglove._protobuf import (
    Pose as _Pose,
    CubePrimitive as _CubePrimitive,
    SceneEntity as _SceneEntity,
    Vector3 as _Vector3,
    Quaternion as _Quaternion,
    Color as _Color,
    ArrowPrimitive as _ArrowPrimitive,
    SpherePrimitive as _SpherePrimitive,
    CylinderPrimitive as _CylinderPrimitive,
    LinePrimitive as _LinePrimitive,
    TriangleListPrimitive as _TriangleListPrimitive,
    TextPrimitive as _TextPrimitive,
    ModelPrimitive as _ModelPrimitive,
    KeyValuePair as _KeyValuePair,
    SceneEntityDeletion as _SceneEntityDeletion,
    SceneUpdate as _SceneUpdate,
    Point3 as _Point3,
    PointCloud as _PointCloud,
    PackedElementField as _PackedElementField,
)

from .timestamps import (
    SdkDuration,
    SdkTimestamp,
    _sdk_duration_to_pb2,
    _sdk_timestamp_to_pb2,
)


class WrappedProto(ABC):
    _proto: Any

    def __bytes__(self) -> bytes:
        # return bytes(self._proto)
        return self._proto.SerializeToString()


class Vector3(WrappedProto):
    def __init__(self, x: float, y: float, z: float):
        self._proto = _Vector3(x=x, y=y, z=z)


class Point3(WrappedProto):
    def __init__(self, x: float, y: float, z: float):
        self._proto = _Point3(x=x, y=y, z=z)


class Quaternion(WrappedProto):
    def __init__(self, x: float, y: float, z: float, w: float):
        self._proto = _Quaternion(x=x, y=y, z=z, w=w)


class Color(WrappedProto):
    def __init__(self, r: float, g: float, b: float, a: float):
        self._proto = _Color(r=r, g=g, b=b, a=a)


class Pose(WrappedProto):
    def __init__(
        self, *, position: Optional[Vector3], orientation: Optional[Quaternion]
    ):
        self._proto = _Pose(
            position=position._proto if position else _Vector3(x=0, y=0, z=0),
            orientation=(
                orientation._proto if orientation else _Quaternion(x=0, y=0, z=0, w=1)
            ),
        )


class CubePrimitive(WrappedProto):
    # This references protos, but would have to keep nesting down.
    def __init__(
        self,
        *,
        pose: Optional[Pose],
        size: Optional[Vector3],
        color: Optional[Color],
    ):
        self._proto = _CubePrimitive(
            pose=(
                pose._proto
                if pose
                else _Pose(
                    position=_Vector3(x=0, y=0, z=0),
                    orientation=_Quaternion(x=0, y=0, z=0, w=1),
                )
            ),
            size=size._proto if size else _Vector3(x=0, y=0, z=0),
            color=color._proto if color else _Color(r=0, g=0, b=0, a=1),
        )


class KeyValuePair(WrappedProto):
    def __init__(self, key: str, value: str):
        self._proto = _KeyValuePair(key=key, value=value)


class ArrowPrimitive(WrappedProto):
    def __init__(
        self,
        *,
        pose: Optional[Pose],
        color: Optional[Color],
        shaft_length: Optional[float],
        shaft_diameter: Optional[float],
        head_length: Optional[float],
        head_diameter: Optional[float],
    ):
        self._proto = _ArrowPrimitive(
            pose=pose._proto if pose else _Pose(),
            color=color._proto if color else _Color(r=0, g=0, b=0, a=1),
            shaft_length=shaft_length or 0,
            shaft_diameter=shaft_diameter or 0,
            head_length=head_length or 0,
            head_diameter=head_diameter or 0,
        )


class SpherePrimitive(WrappedProto):
    def __init__(
        self, *, pose: Optional[Pose], size: Optional[Vector3], color: Optional[Color]
    ):
        self._proto = _SpherePrimitive(
            pose=pose._proto if pose else _Pose(),
            size=size._proto if size else _Vector3(x=0, y=0, z=0),
            color=color._proto if color else _Color(r=0, g=0, b=0, a=1),
        )


class CylinderPrimitive(WrappedProto):
    def __init__(
        self,
        *,
        pose: Optional[Pose],
        size: Optional[Vector3],
        color: Optional[Color],
    ):
        self._proto = _CylinderPrimitive(
            pose=pose._proto if pose else _Pose(),
            size=size._proto if size else _Vector3(x=0, y=0, z=0),
            color=color._proto if color else _Color(r=0, g=0, b=0, a=1),
        )


class LinePrimitive(WrappedProto):
    def __init__(
        self,
        *,
        pose: Optional[Pose],
        color: Optional[Color],
        points: Optional[list[Point3]],
    ):
        self._proto = _LinePrimitive(
            pose=pose._proto if pose else _Pose(),
            color=color._proto if color else _Color(r=0, g=0, b=0, a=1),
            points=[point._proto for point in points] if points else [],
        )


class TriangleListPrimitive(WrappedProto):
    def __init__(
        self,
        *,
        pose: Optional[Pose],
        points: Optional[list[Point3]],
        color: Optional[Color],
        colors: Optional[list[Color]],
        indices: Optional[list[int]],
    ):
        self._proto = _TriangleListPrimitive(
            pose=pose._proto if pose else _Pose(),
            points=[point._proto for point in points] if points else [],
            color=color._proto if color else _Color(r=0, g=0, b=0, a=1),
            colors=[color._proto for color in colors] if colors else [],
            indices=[index for index in indices] if indices else [],
        )


class TextPrimitive(WrappedProto):
    def __init__(
        self,
        *,
        pose: Optional[Pose],
        text: Optional[str],
        color: Optional[Color],
    ):
        self._proto = _TextPrimitive(
            pose=pose._proto if pose else _Pose(),
            text=text or "",
            color=color._proto if color else _Color(r=0, g=0, b=0, a=1),
        )


class ModelPrimitive(WrappedProto):
    def __init__(
        self,
        *,
        pose: Optional[Pose],
        scale: Vector3,
        color: Color,
        override_color: bool,
        url: str,
        media_type: str,
        data: bytes,
    ):
        self._proto = _ModelPrimitive(
            pose=pose._proto if pose else _Pose(),
            scale=scale._proto if scale else _Vector3(x=0, y=0, z=0),
            color=color._proto if color else _Color(r=0, g=0, b=0, a=1),
            override_color=override_color,
            url=url,
            media_type=media_type,
            data=data,
        )


class SceneEntity(WrappedProto):
    def __init__(
        self,
        *,
        timestamp: Optional[SdkTimestamp] = None,
        frame_id: Optional[str] = None,
        id: Optional[str] = None,
        lifetime: Optional[SdkDuration] = None,
        frame_locked: Optional[bool] = False,
        metadata: Optional[list[KeyValuePair]] = None,
        arrows: Optional[list[ArrowPrimitive]] = None,
        cubes: Optional[list[CubePrimitive]] = None,
        spheres: Optional[list[SpherePrimitive]] = None,
        cylinders: Optional[list[CylinderPrimitive]] = None,
        lines: Optional[list[LinePrimitive]] = None,
        triangles: Optional[list[TriangleListPrimitive]] = None,
        texts: Optional[list[TextPrimitive]] = None,
        models: Optional[list[ModelPrimitive]] = None,
    ):
        timestamp = timestamp or datetime.now()
        lifetime = lifetime or timedelta(seconds=0)
        # Timestamp_pb2(seconds=timestamp)

        # if timestamp:
        #     pb_timestamp = Timestamp_pb2(seconds=timestamp.timestamp(), nanos=0)
        # timestamp_pb2 = Timestamp_pb2()
        # timestamp_pb2.FromDatetime(timestamp)
        timestamp_pb2 = _sdk_timestamp_to_pb2(timestamp)
        lifetime_pb2 = _sdk_duration_to_pb2(lifetime)

        self._proto = _SceneEntity(
            frame_id=frame_id or "",
            id=id or "",
            timestamp=timestamp_pb2,
            lifetime=lifetime_pb2,
            # timestamp=timestamp or datetime.now(),
            # lifetime=lifetime or timedelta(seconds=0),
            frame_locked=frame_locked or False,
            metadata=(
                [key_value_pair._proto for key_value_pair in metadata]
                if metadata
                else []
            ),
            arrows=[arrow._proto for arrow in arrows] if arrows else [],
            cubes=[cube._proto for cube in cubes] if cubes else [],
            spheres=[sphere._proto for sphere in spheres] if spheres else [],
            cylinders=[cylinder._proto for cylinder in cylinders] if cylinders else [],
            lines=[line._proto for line in lines] if lines else [],
            triangles=[triangle._proto for triangle in triangles] if triangles else [],
            texts=[text._proto for text in texts] if texts else [],
            models=[model._proto for model in models] if models else [],
        )


class SceneEntityDeletion(WrappedProto):
    def __init__(self, id: str):
        self._proto = _SceneEntityDeletion(id=id)


class SceneUpdate(WrappedProto):
    def __init__(
        self,
        *,
        entities: Optional[list[SceneEntity]] = None,
        deletions: Optional[list[SceneEntityDeletion]] = None,
    ):
        self._proto = _SceneUpdate(
            entities=[entity._proto for entity in entities] if entities else [],
            deletions=[deletion._proto for deletion in deletions] if deletions else [],
        )


class PackedElementField(WrappedProto):
    # https://docs.foxglove.dev/docs/visualization/message-schemas/numeric-type
    class NumericType(Enum):
        UNKNOWN = 0
        UINT8 = 1
        INT8 = 2
        UINT16 = 3
        INT16 = 4
        UINT32 = 5
        INT32 = 6
        FLOAT32 = 7
        FLOAT64 = 8

    def __init__(self, name: str, offset: int, type: NumericType):
        pb_type = {
            PackedElementField.NumericType.UNKNOWN: _PackedElementField.NumericType.UNKNOWN,
            PackedElementField.NumericType.UINT8: _PackedElementField.NumericType.UINT8,
            PackedElementField.NumericType.INT8: _PackedElementField.NumericType.INT8,
            PackedElementField.NumericType.UINT16: _PackedElementField.NumericType.UINT16,
            PackedElementField.NumericType.INT16: _PackedElementField.NumericType.INT16,
            PackedElementField.NumericType.UINT32: _PackedElementField.NumericType.UINT32,
            PackedElementField.NumericType.INT32: _PackedElementField.NumericType.INT32,
            PackedElementField.NumericType.FLOAT32: _PackedElementField.NumericType.FLOAT32,
            PackedElementField.NumericType.FLOAT64: _PackedElementField.NumericType.FLOAT64,
        }.get(type)

        self._proto = _PackedElementField(name=name, offset=offset, type=pb_type)


class PointCloud(WrappedProto):
    def __init__(
        self,
        timestamp: Optional[SdkTimestamp] = None,
        frame_id: Optional[str] = None,
        pose: Optional[Pose] = None,
        point_stride: Optional[int] = None,
        fields: Optional[list[PackedElementField]] = None,
        data: Optional[bytes] = None,
    ):
        self._proto = _PointCloud(
            timestamp=_sdk_timestamp_to_pb2(timestamp) if timestamp else None,
            frame_id=frame_id or "",
            pose=pose._proto if pose else None,
            point_stride=point_stride or 0,
            fields=[field._proto for field in fields] if fields else [],
            data=data or b"",
        )
