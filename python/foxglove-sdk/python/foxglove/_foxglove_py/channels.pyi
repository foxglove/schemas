from . import PartialMetadata
from .schemas import CameraCalibration
from .schemas import CircleAnnotation
from .schemas import Color
from .schemas import CompressedImage
from .schemas import CompressedVideo
from .schemas import FrameTransform
from .schemas import FrameTransforms
from .schemas import GeoJson
from .schemas import Grid
from .schemas import ImageAnnotations
from .schemas import KeyValuePair
from .schemas import LaserScan
from .schemas import LocationFix
from .schemas import Log
from .schemas import SceneEntityDeletion
from .schemas import SceneEntity
from .schemas import SceneUpdate
from .schemas import PackedElementField
from .schemas import Point2
from .schemas import Point3
from .schemas import PointCloud
from .schemas import PointsAnnotation
from .schemas import Pose
from .schemas import PoseInFrame
from .schemas import PosesInFrame
from .schemas import Quaternion
from .schemas import RawImage
from .schemas import TextAnnotation
from .schemas import Vector2
from .schemas import Vector3

class CameraCalibrationChannel:
    """
    A channel for logging CameraCalibration messages
    """

    def __new__(
        cls,
        topic: str,
    ) -> "CameraCalibrationChannel": ...
    def log(
        self,
        message: "CameraCalibration",
    ) -> "CameraCalibrationChannel": ...
    def log_with_meta(
        self,
        message: "CameraCalibration",
        metadata: "PartialMetadata",
    ) -> "CameraCalibrationChannel": ...

class CircleAnnotationChannel:
    """
    A channel for logging CircleAnnotation messages
    """

    def __new__(
        cls,
        topic: str,
    ) -> "CircleAnnotationChannel": ...
    def log(
        self,
        message: "CircleAnnotation",
    ) -> "CircleAnnotationChannel": ...
    def log_with_meta(
        self,
        message: "CircleAnnotation",
        metadata: "PartialMetadata",
    ) -> "CircleAnnotationChannel": ...

class ColorChannel:
    """
    A channel for logging Color messages
    """

    def __new__(
        cls,
        topic: str,
    ) -> "ColorChannel": ...
    def log(
        self,
        message: "Color",
    ) -> "ColorChannel": ...
    def log_with_meta(
        self,
        message: "Color",
        metadata: "PartialMetadata",
    ) -> "ColorChannel": ...

class CompressedImageChannel:
    """
    A channel for logging CompressedImage messages
    """

    def __new__(
        cls,
        topic: str,
    ) -> "CompressedImageChannel": ...
    def log(
        self,
        message: "CompressedImage",
    ) -> "CompressedImageChannel": ...
    def log_with_meta(
        self,
        message: "CompressedImage",
        metadata: "PartialMetadata",
    ) -> "CompressedImageChannel": ...

class CompressedVideoChannel:
    """
    A channel for logging CompressedVideo messages
    """

    def __new__(
        cls,
        topic: str,
    ) -> "CompressedVideoChannel": ...
    def log(
        self,
        message: "CompressedVideo",
    ) -> "CompressedVideoChannel": ...
    def log_with_meta(
        self,
        message: "CompressedVideo",
        metadata: "PartialMetadata",
    ) -> "CompressedVideoChannel": ...

class FrameTransformChannel:
    """
    A channel for logging FrameTransform messages
    """

    def __new__(
        cls,
        topic: str,
    ) -> "FrameTransformChannel": ...
    def log(
        self,
        message: "FrameTransform",
    ) -> "FrameTransformChannel": ...
    def log_with_meta(
        self,
        message: "FrameTransform",
        metadata: "PartialMetadata",
    ) -> "FrameTransformChannel": ...

class FrameTransformsChannel:
    """
    A channel for logging FrameTransforms messages
    """

    def __new__(
        cls,
        topic: str,
    ) -> "FrameTransformsChannel": ...
    def log(
        self,
        message: "FrameTransforms",
    ) -> "FrameTransformsChannel": ...
    def log_with_meta(
        self,
        message: "FrameTransforms",
        metadata: "PartialMetadata",
    ) -> "FrameTransformsChannel": ...

class GeoJsonChannel:
    """
    A channel for logging GeoJson messages
    """

    def __new__(
        cls,
        topic: str,
    ) -> "GeoJsonChannel": ...
    def log(
        self,
        message: "GeoJson",
    ) -> "GeoJsonChannel": ...
    def log_with_meta(
        self,
        message: "GeoJson",
        metadata: "PartialMetadata",
    ) -> "GeoJsonChannel": ...

class GridChannel:
    """
    A channel for logging Grid messages
    """

    def __new__(
        cls,
        topic: str,
    ) -> "GridChannel": ...
    def log(
        self,
        message: "Grid",
    ) -> "GridChannel": ...
    def log_with_meta(
        self,
        message: "Grid",
        metadata: "PartialMetadata",
    ) -> "GridChannel": ...

class ImageAnnotationsChannel:
    """
    A channel for logging ImageAnnotations messages
    """

    def __new__(
        cls,
        topic: str,
    ) -> "ImageAnnotationsChannel": ...
    def log(
        self,
        message: "ImageAnnotations",
    ) -> "ImageAnnotationsChannel": ...
    def log_with_meta(
        self,
        message: "ImageAnnotations",
        metadata: "PartialMetadata",
    ) -> "ImageAnnotationsChannel": ...

class KeyValuePairChannel:
    """
    A channel for logging KeyValuePair messages
    """

    def __new__(
        cls,
        topic: str,
    ) -> "KeyValuePairChannel": ...
    def log(
        self,
        message: "KeyValuePair",
    ) -> "KeyValuePairChannel": ...
    def log_with_meta(
        self,
        message: "KeyValuePair",
        metadata: "PartialMetadata",
    ) -> "KeyValuePairChannel": ...

class LaserScanChannel:
    """
    A channel for logging LaserScan messages
    """

    def __new__(
        cls,
        topic: str,
    ) -> "LaserScanChannel": ...
    def log(
        self,
        message: "LaserScan",
    ) -> "LaserScanChannel": ...
    def log_with_meta(
        self,
        message: "LaserScan",
        metadata: "PartialMetadata",
    ) -> "LaserScanChannel": ...

class LocationFixChannel:
    """
    A channel for logging LocationFix messages
    """

    def __new__(
        cls,
        topic: str,
    ) -> "LocationFixChannel": ...
    def log(
        self,
        message: "LocationFix",
    ) -> "LocationFixChannel": ...
    def log_with_meta(
        self,
        message: "LocationFix",
        metadata: "PartialMetadata",
    ) -> "LocationFixChannel": ...

class LogChannel:
    """
    A channel for logging Log messages
    """

    def __new__(
        cls,
        topic: str,
    ) -> "LogChannel": ...
    def log(
        self,
        message: "Log",
    ) -> "LogChannel": ...
    def log_with_meta(
        self,
        message: "Log",
        metadata: "PartialMetadata",
    ) -> "LogChannel": ...

class PackedElementFieldChannel:
    """
    A channel for logging PackedElementField messages
    """

    def __new__(
        cls,
        topic: str,
    ) -> "PackedElementFieldChannel": ...
    def log(
        self,
        message: "PackedElementField",
    ) -> "PackedElementFieldChannel": ...
    def log_with_meta(
        self,
        message: "PackedElementField",
        metadata: "PartialMetadata",
    ) -> "PackedElementFieldChannel": ...

class Point2Channel:
    """
    A channel for logging Point2 messages
    """

    def __new__(
        cls,
        topic: str,
    ) -> "Point2Channel": ...
    def log(
        self,
        message: "Point2",
    ) -> "Point2Channel": ...
    def log_with_meta(
        self,
        message: "Point2",
        metadata: "PartialMetadata",
    ) -> "Point2Channel": ...

class Point3Channel:
    """
    A channel for logging Point3 messages
    """

    def __new__(
        cls,
        topic: str,
    ) -> "Point3Channel": ...
    def log(
        self,
        message: "Point3",
    ) -> "Point3Channel": ...
    def log_with_meta(
        self,
        message: "Point3",
        metadata: "PartialMetadata",
    ) -> "Point3Channel": ...

class PointCloudChannel:
    """
    A channel for logging PointCloud messages
    """

    def __new__(
        cls,
        topic: str,
    ) -> "PointCloudChannel": ...
    def log(
        self,
        message: "PointCloud",
    ) -> "PointCloudChannel": ...
    def log_with_meta(
        self,
        message: "PointCloud",
        metadata: "PartialMetadata",
    ) -> "PointCloudChannel": ...

class PointsAnnotationChannel:
    """
    A channel for logging PointsAnnotation messages
    """

    def __new__(
        cls,
        topic: str,
    ) -> "PointsAnnotationChannel": ...
    def log(
        self,
        message: "PointsAnnotation",
    ) -> "PointsAnnotationChannel": ...
    def log_with_meta(
        self,
        message: "PointsAnnotation",
        metadata: "PartialMetadata",
    ) -> "PointsAnnotationChannel": ...

class PoseChannel:
    """
    A channel for logging Pose messages
    """

    def __new__(
        cls,
        topic: str,
    ) -> "PoseChannel": ...
    def log(
        self,
        message: "Pose",
    ) -> "PoseChannel": ...
    def log_with_meta(
        self,
        message: "Pose",
        metadata: "PartialMetadata",
    ) -> "PoseChannel": ...

class PoseInFrameChannel:
    """
    A channel for logging PoseInFrame messages
    """

    def __new__(
        cls,
        topic: str,
    ) -> "PoseInFrameChannel": ...
    def log(
        self,
        message: "PoseInFrame",
    ) -> "PoseInFrameChannel": ...
    def log_with_meta(
        self,
        message: "PoseInFrame",
        metadata: "PartialMetadata",
    ) -> "PoseInFrameChannel": ...

class PosesInFrameChannel:
    """
    A channel for logging PosesInFrame messages
    """

    def __new__(
        cls,
        topic: str,
    ) -> "PosesInFrameChannel": ...
    def log(
        self,
        message: "PosesInFrame",
    ) -> "PosesInFrameChannel": ...
    def log_with_meta(
        self,
        message: "PosesInFrame",
        metadata: "PartialMetadata",
    ) -> "PosesInFrameChannel": ...

class QuaternionChannel:
    """
    A channel for logging Quaternion messages
    """

    def __new__(
        cls,
        topic: str,
    ) -> "QuaternionChannel": ...
    def log(
        self,
        message: "Quaternion",
    ) -> "QuaternionChannel": ...
    def log_with_meta(
        self,
        message: "Quaternion",
        metadata: "PartialMetadata",
    ) -> "QuaternionChannel": ...

class RawImageChannel:
    """
    A channel for logging RawImage messages
    """

    def __new__(
        cls,
        topic: str,
    ) -> "RawImageChannel": ...
    def log(
        self,
        message: "RawImage",
    ) -> "RawImageChannel": ...
    def log_with_meta(
        self,
        message: "RawImage",
        metadata: "PartialMetadata",
    ) -> "RawImageChannel": ...

class SceneEntityChannel:
    """
    A channel for logging SceneEntity messages
    """

    def __new__(
        cls,
        topic: str,
    ) -> "SceneEntityChannel": ...
    def log(
        self,
        message: "SceneEntity",
    ) -> "SceneEntityChannel": ...
    def log_with_meta(
        self,
        message: "SceneEntity",
        metadata: "PartialMetadata",
    ) -> "SceneEntityChannel": ...

class SceneEntityDeletionChannel:
    """
    A channel for logging SceneEntityDeletion messages
    """

    def __new__(
        cls,
        topic: str,
    ) -> "SceneEntityDeletionChannel": ...
    def log(
        self,
        message: "SceneEntityDeletion",
    ) -> "SceneEntityDeletionChannel": ...
    def log_with_meta(
        self,
        message: "SceneEntityDeletion",
        metadata: "PartialMetadata",
    ) -> "SceneEntityDeletionChannel": ...

class SceneUpdateChannel:
    """
    A channel for logging SceneUpdate messages
    """

    def __new__(
        cls,
        topic: str,
    ) -> "SceneUpdateChannel": ...
    def log(
        self,
        message: "SceneUpdate",
    ) -> "SceneUpdateChannel": ...
    def log_with_meta(
        self,
        message: "SceneUpdate",
        metadata: "PartialMetadata",
    ) -> "SceneUpdateChannel": ...

class TextAnnotationChannel:
    """
    A channel for logging TextAnnotation messages
    """

    def __new__(
        cls,
        topic: str,
    ) -> "TextAnnotationChannel": ...
    def log(
        self,
        message: "TextAnnotation",
    ) -> "TextAnnotationChannel": ...
    def log_with_meta(
        self,
        message: "TextAnnotation",
        metadata: "PartialMetadata",
    ) -> "TextAnnotationChannel": ...

class Vector2Channel:
    """
    A channel for logging Vector2 messages
    """

    def __new__(
        cls,
        topic: str,
    ) -> "Vector2Channel": ...
    def log(
        self,
        message: "Vector2",
    ) -> "Vector2Channel": ...
    def log_with_meta(
        self,
        message: "Vector2",
        metadata: "PartialMetadata",
    ) -> "Vector2Channel": ...

class Vector3Channel:
    """
    A channel for logging Vector3 messages
    """

    def __new__(
        cls,
        topic: str,
    ) -> "Vector3Channel": ...
    def log(
        self,
        message: "Vector3",
    ) -> "Vector3Channel": ...
    def log_with_meta(
        self,
        message: "Vector3",
        metadata: "PartialMetadata",
    ) -> "Vector3Channel": ...
