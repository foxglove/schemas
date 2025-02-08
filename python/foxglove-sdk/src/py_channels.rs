//! Schema-specific channels for logging FG schemas.
//! We'll beed to generate pyclasses for each rather than using TypedChannel;
//! see https://pyo3.rs/v0.23.4/class.html#no-generic-parameters
//! Spike: serialize python messages on the rust side

use crate::errors::PyFoxgloveError;
use bytes::Bytes;
use foxglove::schemas::{PointCloud, SceneUpdate};
use foxglove::{Channel, ChannelBuilder, Encode, PartialMetadata, TypedChannel};
use prost::bytes;
use pyo3::types::{PyBytes, PyList};
use pyo3::{prelude::*, py_run};
use std::borrow::Borrow;
use std::collections::BTreeMap;
use std::fs::metadata;
use std::sync::Arc;

// todo: may want TypedChannel here, but then we need to generate concrete pyclasses for each schema
// since pyo3 doesn't support generics
#[pyclass]
pub(crate) struct BaseSceneUpdateChannel(TypedChannel<SceneUpdate>);

#[pymethods]
impl BaseSceneUpdateChannel {
    #[new]
    #[pyo3(
        signature = (topic, message_encoding, metadata=None)
    )]
    fn new(
        topic: &str,
        message_encoding: &str,
        metadata: Option<BTreeMap<String, String>>,
    ) -> PyResult<Self> {
        let schema = foxglove::schemas::SceneUpdate::get_schema().unwrap();

        let channel = ChannelBuilder::new(topic)
            .message_encoding(message_encoding)
            .schema(schema)
            .metadata(metadata.unwrap_or_default())
            .build_typed()
            .map_err(PyFoxgloveError::from)?;

        Ok(BaseSceneUpdateChannel(channel))
    }

    #[pyo3(signature = (msg, publish_time=None, log_time=None, sequence=None))]
    fn log(
        &self,
        // see https://pyo3.rs/v0.23.4/class.html#no-generic-parameters -- need to generate
        // concrete classes for each schema
        msg: crate::py_schemas::SceneUpdate,
        publish_time: Option<u64>,
        log_time: Option<u64>,
        sequence: Option<u32>,
    ) -> PyResult<()> {
        // Encode fg schemas
        let update = foxglove::schemas::SceneUpdate::from(msg);

        let metadata = PartialMetadata {
            sequence,
            log_time,
            publish_time,
        };
        self.0.log_with_meta(&update, metadata);
        Ok(())
    }
}

#[pyclass]
pub(crate) struct BasePointCloudChannel(TypedChannel<PointCloud>);

#[pymethods]
impl BasePointCloudChannel {
    #[new]
    #[pyo3(
        signature = (topic, message_encoding, metadata=None)
    )]
    fn new(
        topic: &str,
        message_encoding: &str,
        metadata: Option<BTreeMap<String, String>>,
    ) -> PyResult<Self> {
        let schema = foxglove::schemas::PointCloud::get_schema().unwrap();

        let channel = ChannelBuilder::new(topic)
            .message_encoding(message_encoding)
            .schema(schema)
            .metadata(metadata.unwrap_or_default())
            .build_typed()
            .map_err(PyFoxgloveError::from)?;

        Ok(BasePointCloudChannel(channel))
    }

    #[pyo3(signature = (msg, publish_time=None, log_time=None, sequence=None))]
    fn log(
        &self,
        // see https://pyo3.rs/v0.23.4/class.html#no-generic-parameters -- need to generate
        // concrete classes for each schema
        msg: crate::py_schemas::PointCloud,
        publish_time: Option<u64>,
        log_time: Option<u64>,
        sequence: Option<u32>,
    ) -> PyResult<()> {
        // Pre-allocate vectors to avoid reallocations
        let mut fields = Vec::with_capacity(msg.fields.len());
        fields.extend(msg.fields.into_iter().map(|f| f.into()));

        // Move data directly without cloning
        let point_cloud = foxglove::schemas::PointCloud {
            timestamp: Some(msg.timestamp.into()),
            frame_id: msg.frame_id,
            pose: Some(msg.pose.into()),
            point_stride: msg.point_stride,
            fields,
            data: msg.data.into(),
        };

        let metadata = PartialMetadata {
            sequence,
            log_time,
            publish_time,
        };
        self.0.log_with_meta(&point_cloud, metadata);
        Ok(())
    }
}

#[pyclass]
pub(crate) struct OptimizedPointCloudChannel(TypedChannel<PointCloud>);

#[pymethods]
impl OptimizedPointCloudChannel {
    /// WIP: trying to get a no-copy version of PointCloudChannel working
    #[new]
    #[pyo3(
        signature = (topic, message_encoding, metadata=None)
    )]
    fn new(
        topic: &str,
        message_encoding: &str,
        metadata: Option<BTreeMap<String, String>>,
    ) -> PyResult<Self> {
        let schema = foxglove::schemas::PointCloud::get_schema().unwrap();

        let channel = ChannelBuilder::new(topic)
            .message_encoding(message_encoding)
            .schema(schema)
            .metadata(metadata.unwrap_or_default())
            .build_typed()
            .map_err(PyFoxgloveError::from)?;

        Ok(OptimizedPointCloudChannel(channel))
    }

    #[pyo3(signature = (msg, publish_time=None, log_time=None, sequence=None))]
    fn log<'py>(
        &self,
        msg: Bound<'py, crate::py_schemas::OptimizedPointCloud>,
        publish_time: Option<u64>,
        log_time: Option<u64>,
        sequence: Option<u32>,
    ) -> PyResult<()> {
        let msg_ref = msg.borrow();
        let py = msg.py();

        // Get bytes reference without copying
        let bytes = msg_ref.data.as_bytes(py);
        // Avoid double allocation by pre-allocating the vector
        let mut data = Vec::with_capacity(bytes.len());
        data.extend_from_slice(bytes);

        // Create the point cloud with minimal copying
        let point_cloud = foxglove::schemas::PointCloud {
            timestamp: None,
            frame_id: String::new(),
            pose: Some(msg_ref.pose.clone().into()),
            point_stride: msg_ref.point_stride,
            fields: msg_ref.fields.iter().map(|f| f.clone().into()).collect(),
            data: data.into(),
        };

        let metadata = PartialMetadata {
            sequence,
            log_time,
            publish_time,
        };

        // Use TypedChannel's log_with_meta which handles the encoding
        self.0.log_with_meta(&point_cloud, metadata);

        Ok(())
    }
}

// testing out a bytearray directly.
// I think the copy here can be avoided by having our Prost types take `Bytes`
#[pyfunction]
pub(crate) fn log_bytes<'py>(bytes: Bound<'py, PyBytes>) -> PyResult<()> {
    println!("[rust] bytes: {:?}", bytes);

    // Pre-allocate vector to avoid reallocation
    let bytes_ref = bytes.borrow().as_bytes();
    let mut data = Vec::with_capacity(bytes_ref.len());
    data.extend_from_slice(bytes_ref);

    let point_cloud = foxglove::schemas::PointCloud {
        timestamp: None,
        frame_id: String::new(),
        pose: None,
        point_stride: 0,
        fields: Vec::new(),
        data: data.into(),
    };

    println!("point_cloud: {:?}", point_cloud);
    Ok(())
}

// test with a top-level function.
// This lets us use `Bound` — pyclasses can't have lifetimes —
// but we can still use pyclasses and do the binding ourselves.
// So this could be an API addition, but isn't required.
#[pyfunction]
pub(crate) fn log_point_cloud<'py>(
    channel: Bound<'py, BasePointCloudChannel>,
    point_cloud: Bound<'py, crate::py_schemas::PointCloud>,
    list: Bound<'py, PyList>,
) -> PyResult<()> {
    let point_cloud = point_cloud.borrow();

    // Pre-allocate vectors to avoid reallocations
    let mut fields = Vec::with_capacity(point_cloud.fields.len());
    fields.extend(point_cloud.fields.iter().map(|f| f.clone().into()));

    // Create point cloud with minimal copying
    let cloud = foxglove::schemas::PointCloud {
        timestamp: Some(point_cloud.timestamp.clone().into()),
        frame_id: point_cloud.frame_id.clone(),
        pose: Some(point_cloud.pose.clone().into()),
        point_stride: point_cloud.point_stride,
        fields,
        data: point_cloud.data.clone().into(),
    };

    // Log the point cloud
    channel
        .borrow()
        .0
        .log_with_meta(&cloud, PartialMetadata::default());

    for item in list.iter() {
        println!("item: {:?}", item);
    }

    Ok(())
}
