use pyo3::prelude::*;

#[pymodule]
mod schemas {
    use pyo3::types::PyAnyMethods;
    use pyo3::types::PyModule;
    use pyo3::Bound;
    use pyo3::PyResult;

    #[pymodule_export]
    use crate::schemas::Timestamp;

    #[pymodule_export]
    use crate::schemas::Duration;

    #[pymodule_export]
    use crate::schemas::LinePrimitiveLineType;

    #[pymodule_export]
    use crate::schemas::LogLevel;

    #[pymodule_export]
    use crate::schemas::SceneEntityDeletionType;

    #[pymodule_export]
    use crate::schemas::PackedElementFieldNumericType;

    #[pymodule_export]
    use crate::schemas::PointsAnnotationType;

    #[pymodule_export]
    use crate::schemas::LocationFixPositionCovarianceType;

    #[pymodule_export]
    use crate::schemas::ArrowPrimitive;

    #[pymodule_export]
    use crate::schemas::CameraCalibration;

    #[pymodule_export]
    use crate::schemas::CircleAnnotation;

    #[pymodule_export]
    use crate::schemas::Color;

    #[pymodule_export]
    use crate::schemas::CompressedImage;

    #[pymodule_export]
    use crate::schemas::CompressedVideo;

    #[pymodule_export]
    use crate::schemas::CylinderPrimitive;

    #[pymodule_export]
    use crate::schemas::CubePrimitive;

    #[pymodule_export]
    use crate::schemas::FrameTransform;

    #[pymodule_export]
    use crate::schemas::FrameTransforms;

    #[pymodule_export]
    use crate::schemas::GeoJson;

    #[pymodule_export]
    use crate::schemas::Grid;

    #[pymodule_export]
    use crate::schemas::ImageAnnotations;

    #[pymodule_export]
    use crate::schemas::KeyValuePair;

    #[pymodule_export]
    use crate::schemas::LaserScan;

    #[pymodule_export]
    use crate::schemas::LinePrimitive;

    #[pymodule_export]
    use crate::schemas::LocationFix;

    #[pymodule_export]
    use crate::schemas::Log;

    #[pymodule_export]
    use crate::schemas::SceneEntityDeletion;

    #[pymodule_export]
    use crate::schemas::SceneEntity;

    #[pymodule_export]
    use crate::schemas::SceneUpdate;

    #[pymodule_export]
    use crate::schemas::ModelPrimitive;

    #[pymodule_export]
    use crate::schemas::PackedElementField;

    #[pymodule_export]
    use crate::schemas::Point2;

    #[pymodule_export]
    use crate::schemas::Point3;

    #[pymodule_export]
    use crate::schemas::PointCloud;

    #[pymodule_export]
    use crate::schemas::PointsAnnotation;

    #[pymodule_export]
    use crate::schemas::Pose;

    #[pymodule_export]
    use crate::schemas::PoseInFrame;

    #[pymodule_export]
    use crate::schemas::PosesInFrame;

    #[pymodule_export]
    use crate::schemas::Quaternion;

    #[pymodule_export]
    use crate::schemas::RawImage;

    #[pymodule_export]
    use crate::schemas::SpherePrimitive;

    #[pymodule_export]
    use crate::schemas::TextAnnotation;

    #[pymodule_export]
    use crate::schemas::TextPrimitive;

    #[pymodule_export]
    use crate::schemas::TriangleListPrimitive;

    #[pymodule_export]
    use crate::schemas::Vector2;

    #[pymodule_export]
    use crate::schemas::Vector3;

    #[pymodule_init]
    fn init(m: &Bound<'_, PyModule>) -> PyResult<()> {
        // Define as a package
        // https://github.com/PyO3/pyo3/issues/759
        let py = m.py();
        py.import("sys")?
            .getattr("modules")?
            .set_item("foxglove._foxglove_py.schemas", m)
    }
}
