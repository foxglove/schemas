//! Schema definitions exposed as pyclasses
//! Spike: serialize python messages on the rust side
use prost_types;
use pyo3::{
    prelude::*,
    types::{PyBytes, PyList},
};
// use pyo3::types::PyList;

#[pyclass]
#[derive(Clone)]
pub(crate) struct Vector3 {
    x: f64,
    y: f64,
    z: f64,
}

#[pymethods]
impl Vector3 {
    #[new]
    fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct Point3 {
    x: f64,
    y: f64,
    z: f64,
}

#[pymethods]
impl Point3 {
    #[new]
    fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct Quaternion {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

#[pymethods]
impl Quaternion {
    #[new]
    fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self { x, y, z, w }
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct Color {
    r: f64,
    g: f64,
    b: f64,
    a: f64,
}

#[pymethods]
impl Color {
    #[new]
    fn new(r: f64, g: f64, b: f64, a: f64) -> Self {
        Self { r, g, b, a }
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct Pose {
    pub(crate) position: Option<Vector3>,
    pub(crate) orientation: Option<Quaternion>,
}

#[pymethods]
impl Pose {
    #[new]
    fn new(position: PyRef<Vector3>, orientation: PyRef<Quaternion>) -> Self {
        Self {
            position: Some(Vector3 {
                x: position.x,
                y: position.y,
                z: position.z,
            }),
            orientation: Some(Quaternion {
                x: orientation.x,
                y: orientation.y,
                z: orientation.z,
                w: orientation.w,
            }),
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct CubePrimitive {
    pose: Pose,
    size: Vector3,
    color: Color,
}

// todo: avoid cloning here and everywhere
#[pymethods]
impl CubePrimitive {
    #[new]
    fn new(pose: PyRef<Pose>, size: PyRef<Vector3>, color: PyRef<Color>) -> Self {
        Self {
            pose: pose.clone(),
            size: size.clone(),
            color: color.clone(),
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct KeyValuePair {
    key: String,
    value: String,
}

#[pymethods]
impl KeyValuePair {
    #[new]
    fn new(key: String, value: String) -> Self {
        Self { key, value }
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct ArrowPrimitive {
    pose: Pose,
    color: Color,
    shaft_length: f64,
    shaft_diameter: f64,
    head_length: f64,
    head_diameter: f64,
}

#[pymethods]
impl ArrowPrimitive {
    #[new]
    fn new(
        pose: PyRef<Pose>,
        color: PyRef<Color>,
        shaft_length: f64,
        shaft_diameter: f64,
        head_length: f64,
        head_diameter: f64,
    ) -> Self {
        Self {
            pose: pose.clone(),
            color: color.clone(),
            shaft_length,
            shaft_diameter,
            head_length,
            head_diameter,
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct SpherePrimitive {
    pose: Pose,
    size: Vector3,
    color: Color,
}

#[pymethods]
impl SpherePrimitive {
    #[new]
    fn new(pose: Pose, size: Vector3, color: Color) -> Self {
        Self { pose, size, color }
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct CylinderPrimitive {
    pose: Pose,
    size: Vector3,
    color: Color,
}

#[pymethods]
impl CylinderPrimitive {
    #[new]
    fn new(pose: Pose, size: Vector3, color: Color) -> Self {
        Self { pose, size, color }
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct LinePrimitive {
    pose: Pose,
    color: Color,
    points: Vec<Point3>,
}

#[pymethods]
impl LinePrimitive {
    #[new]
    fn new(pose: Pose, color: Color, points: Vec<Point3>) -> Self {
        Self {
            pose,
            color,
            points,
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct TriangleListPrimitive {
    pose: Pose,
    points: Vec<Point3>,
    color: Color,
    colors: Vec<Color>,
    indices: Vec<i32>,
}

#[pymethods]
impl TriangleListPrimitive {
    #[new]
    fn new(
        pose: Pose,
        points: Vec<Point3>,
        color: Color,
        colors: Vec<Color>,
        indices: Vec<i32>,
    ) -> Self {
        Self {
            pose,
            points,
            color,
            colors,
            indices,
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct TextPrimitive {
    pose: Pose,
    text: String,
    color: Color,
}

#[pymethods]
impl TextPrimitive {
    #[new]
    fn new(pose: Pose, text: String, color: Color) -> Self {
        Self { pose, text, color }
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct ModelPrimitive {
    pose: Pose,
    scale: Vector3,
    color: Color,
    override_color: bool,
    url: String,
    media_type: String,
    data: Vec<u8>,
}

#[pymethods]
impl ModelPrimitive {
    #[new]
    fn new(
        pose: Pose,
        scale: Vector3,
        color: Color,
        override_color: bool,
        url: String,
        media_type: String,
        data: Vec<u8>,
    ) -> Self {
        Self {
            pose,
            scale,
            color,
            override_color,
            url,
            media_type,
            data,
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct Timestamp {
    seconds: i64,
    nanos: i32,
}

#[pymethods]
impl Timestamp {
    #[new]
    fn new(seconds: i64, nanos: i32) -> Self {
        Self { seconds, nanos }
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct Duration {
    seconds: i64,
    nanos: i32,
}

#[pymethods]
impl Duration {
    #[new]
    fn new(seconds: i64, nanos: i32) -> Self {
        Self { seconds, nanos }
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct SceneEntity {
    frame_id: String,
    id: String,
    timestamp: Timestamp,
    lifetime: Duration,
    frame_locked: bool,
    cubes: Vec<CubePrimitive>,
    // metadata: Vec<KeyValuePair>,
    // arrows: Vec<ArrowPrimitive>,
    // spheres: Vec<SpherePrimitive>,
    // cylinders: Vec<CylinderPrimitive>,
    // lines: Vec<LinePrimitive>,
    // triangles: Vec<TriangleListPrimitive>,
    // texts: Vec<TextPrimitive>,
    // models: Vec<ModelPrimitive>,
}

#[pymethods]
impl SceneEntity {
    #[new]
    fn new(
        frame_id: String,
        id: String,
        timestamp: PyRef<Timestamp>,
        lifetime: PyRef<Duration>,
        frame_locked: bool,
        cubes: Vec<CubePrimitive>,
        // metadata: Vec<KeyValuePair>,
        // arrows: Vec<ArrowPrimitive>,
        // spheres: Vec<SpherePrimitive>,
        // cylinders: Vec<CylinderPrimitive>,
        // lines: Vec<LinePrimitive>,
        // triangles: Vec<TriangleListPrimitive>,
        // texts: Vec<TextPrimitive>,
        // models: Vec<ModelPrimitive>,
    ) -> Self {
        Self {
            frame_id,
            id,
            timestamp: timestamp.clone(),
            lifetime: lifetime.clone(),
            frame_locked,
            cubes: cubes.clone(),
            // metadata,
            // arrows,
            // spheres,
            // cylinders,
            // lines,
            // triangles,
            // texts,
            // models,
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct SceneEntityDeletion {
    id: String,
}

#[pymethods]
impl SceneEntityDeletion {
    #[new]
    fn new(id: String) -> Self {
        Self { id }
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct SceneUpdate {
    entities: Vec<SceneEntity>,
    deletions: Vec<SceneEntityDeletion>,
}

#[pymethods]
impl SceneUpdate {
    #[new]
    fn new(entities: Vec<SceneEntity>, deletions: Vec<SceneEntityDeletion>) -> Self {
        Self {
            entities,
            deletions,
        }
    }
}

#[pyclass(eq, eq_int)]
#[derive(PartialEq, Clone)]
pub(crate) enum PackedElementFieldType {
    Unknown = 0,
    Uint8 = 1,
    Int8 = 2,
    Uint16 = 3,
    Int16 = 4,
    Uint32 = 5,
    Int32 = 6,
    Float32 = 7,
    Float64 = 8,
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct PackedElementField {
    name: String,
    offset: u32,
    r#type: PackedElementFieldType,
}

#[pymethods]
impl PackedElementField {
    #[new]
    fn new(name: String, offset: u32, r#type: PackedElementFieldType) -> Self {
        Self {
            name,
            offset,
            r#type,
        }
    }
}

#[pyclass]
pub(crate) struct OptimizedPointCloud {
    //timestamp: Timestamp,
    //frame_id: String,
    pub(crate) pose: Pose,
    pub(crate) point_stride: u32,
    pub(crate) fields: Py<PyList>,
    pub(crate) data: Py<PyBytes>,
}

#[pymethods]
impl OptimizedPointCloud {
    #[new]
    fn new(pose: Pose, point_stride: u32, fields: Py<PyList>, data: Py<PyBytes>) -> Self {
        Self {
            //timestamp: Timestamp::new(0, 0),
            //frame_id: "".to_string(),
            pose,
            point_stride,
            fields,
            data,
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct PointCloud {
    timestamp: Timestamp,
    frame_id: String,
    pose: Pose,
    point_stride: u32,
    fields: Vec<PackedElementField>,
    data: Vec<u8>,
}

#[pymethods]
impl PointCloud {
    #[new]
    fn new(
        timestamp: Timestamp,
        frame_id: String,
        pose: Pose,
        point_stride: u32,
        fields: Vec<PackedElementField>,
        data: Vec<u8>,
    ) -> Self {
        Self {
            timestamp,
            frame_id,
            pose,
            point_stride,
            fields,
            data,
        }
    }
}

/// conversions

impl From<Timestamp> for prost_types::Timestamp {
    fn from(value: Timestamp) -> Self {
        Self {
            seconds: value.seconds,
            nanos: value.nanos,
        }
    }
}

impl From<Duration> for prost_types::Duration {
    fn from(value: Duration) -> Self {
        Self {
            seconds: value.seconds,
            nanos: value.nanos,
        }
    }
}

impl From<Vector3> for foxglove::schemas::Vector3 {
    fn from(value: Vector3) -> Self {
        Self {
            x: value.x,
            y: value.y,
            z: value.z,
        }
    }
}

impl From<Quaternion> for foxglove::schemas::Quaternion {
    fn from(value: Quaternion) -> Self {
        Self {
            x: value.x,
            y: value.y,
            z: value.z,
            w: value.w,
        }
    }
}

impl From<Color> for foxglove::schemas::Color {
    fn from(value: Color) -> Self {
        Self {
            r: value.r,
            g: value.g,
            b: value.b,
            a: value.a,
        }
    }
}

impl From<Pose> for foxglove::schemas::Pose {
    fn from(value: Pose) -> Self {
        Self {
            position: value.position.map(|p| p.into()),
            orientation: value.orientation.map(|p| p.into()),
        }
    }
}

impl From<CubePrimitive> for foxglove::schemas::CubePrimitive {
    fn from(value: CubePrimitive) -> Self {
        Self {
            pose: Some(value.pose.into()),
            size: Some(value.size.into()),
            color: Some(value.color.into()),
        }
    }
}

impl From<SceneEntity> for foxglove::schemas::SceneEntity {
    fn from(value: SceneEntity) -> Self {
        Self {
            frame_id: value.frame_id,
            id: value.id,
            timestamp: Some(value.timestamp.into()),
            lifetime: Some(value.lifetime.into()),
            frame_locked: value.frame_locked,
            cubes: value.cubes.into_iter().map(|c| c.into()).collect(),
            // todo: all these fields
            metadata: vec![],
            arrows: vec![],
            spheres: vec![],
            cylinders: vec![],
            lines: vec![],
            triangles: vec![],
            texts: vec![],
            models: vec![],
        }
    }
}

impl From<SceneEntityDeletion> for foxglove::schemas::SceneEntityDeletion {
    fn from(value: SceneEntityDeletion) -> Self {
        Self {
            id: value.id,
            // todo
            timestamp: None,
            r#type: 0,
        }
    }
}

impl From<SceneUpdate> for foxglove::schemas::SceneUpdate {
    fn from(value: SceneUpdate) -> Self {
        Self {
            entities: value.entities.into_iter().map(|e| e.into()).collect(),
            deletions: value.deletions.into_iter().map(|d| d.into()).collect(),
        }
    }
}

impl From<PointCloud> for foxglove::schemas::PointCloud {
    fn from(value: PointCloud) -> Self {
        Self {
            timestamp: Some(value.timestamp.into()),
            frame_id: value.frame_id,
            pose: Some(value.pose.into()),
            point_stride: value.point_stride,
            fields: value.fields.into_iter().map(|f| f.into()).collect(),
            data: bytes::Bytes::from(value.data),
        }
    }
}

impl From<PackedElementField> for foxglove::schemas::PackedElementField {
    fn from(value: PackedElementField) -> Self {
        Self {
            name: value.name,
            offset: value.offset,
            r#type: value.r#type as i32,
        }
    }
}
