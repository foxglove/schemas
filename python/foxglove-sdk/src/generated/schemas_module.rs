#[pymodule]
mod schemas {
    use pyo3::types::PyAnyMethods;
    use pyo3::types::PyModule;
    use pyo3::Bound;
    use pyo3::PyResult;

    #[pymodule_export]
    use super::py_schemas::Timestamp;

    #[pymodule_export]
    use super::py_schemas::Duration;

    #[pymodule_export]
    use super::py_schemas::LinePrimitiveLineType;

    #[pymodule_export]
    use super::py_schemas::LogLevel;

    #[pymodule_export]
    use super::py_schemas::SceneEntityDeletionType;

    #[pymodule_export]
    use super::py_schemas::PackedElementFieldNumericType;

    #[pymodule_export]
    use super::py_schemas::PointsAnnotationType;

    #[pymodule_export]
    use super::py_schemas::LocationFixPositionCovarianceType;

    #[pymodule_export]
    use super::py_schemas::ArrowPrimitive;

    #[pymodule_export]
    use super::py_schemas::CameraCalibration;

    #[pymodule_export]
    use super::py_schemas::CircleAnnotation;

    #[pymodule_export]
    use super::py_schemas::Color;

    #[pymodule_export]
    use super::py_schemas::CompressedImage;

    #[pymodule_export]
    use super::py_schemas::CompressedVideo;

    #[pymodule_export]
    use super::py_schemas::CylinderPrimitive;

    #[pymodule_export]
    use super::py_schemas::CubePrimitive;

    #[pymodule_export]
    use super::py_schemas::FrameTransform;

    #[pymodule_export]
    use super::py_schemas::FrameTransforms;

    #[pymodule_export]
    use super::py_schemas::GeoJson;

    #[pymodule_export]
    use super::py_schemas::Grid;

    #[pymodule_export]
    use super::py_schemas::ImageAnnotations;

    #[pymodule_export]
    use super::py_schemas::KeyValuePair;

    #[pymodule_export]
    use super::py_schemas::LaserScan;

    #[pymodule_export]
    use super::py_schemas::LinePrimitive;

    #[pymodule_export]
    use super::py_schemas::LocationFix;

    #[pymodule_export]
    use super::py_schemas::Log;

    #[pymodule_export]
    use super::py_schemas::SceneEntityDeletion;

    #[pymodule_export]
    use super::py_schemas::SceneEntity;

    #[pymodule_export]
    use super::py_schemas::SceneUpdate;

    #[pymodule_export]
    use super::py_schemas::ModelPrimitive;

    #[pymodule_export]
    use super::py_schemas::PackedElementField;

    #[pymodule_export]
    use super::py_schemas::Point2;

    #[pymodule_export]
    use super::py_schemas::Point3;

    #[pymodule_export]
    use super::py_schemas::PointCloud;

    #[pymodule_export]
    use super::py_schemas::PointsAnnotation;

    #[pymodule_export]
    use super::py_schemas::Pose;

    #[pymodule_export]
    use super::py_schemas::PoseInFrame;

    #[pymodule_export]
    use super::py_schemas::PosesInFrame;

    #[pymodule_export]
    use super::py_schemas::Quaternion;

    #[pymodule_export]
    use super::py_schemas::RawImage;

    #[pymodule_export]
    use super::py_schemas::SpherePrimitive;

    #[pymodule_export]
    use super::py_schemas::TextAnnotation;

    #[pymodule_export]
    use super::py_schemas::TextPrimitive;

    #[pymodule_export]
    use super::py_schemas::TriangleListPrimitive;

    #[pymodule_export]
    use super::py_schemas::Vector2;

    #[pymodule_export]
    use super::py_schemas::Vector3;

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
