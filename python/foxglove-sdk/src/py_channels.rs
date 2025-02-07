//! Schema-specific channels for logging FG schemas.
//! We'll beed to generate pyclasses for each rather than using TypedChannel;
//! see https://pyo3.rs/v0.23.4/class.html#no-generic-parameters
//! Spike: serialize python messages on the rust side

use crate::errors::PyFoxgloveError;
use foxglove::schemas::{PointCloud, SceneUpdate};
use foxglove::{Channel, ChannelBuilder, Encode, PartialMetadata, TypedChannel};
use pyo3::types::{PyBytes, PyList};
use pyo3::{prelude::*, py_run};
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
        // Encode fg schemas
        let update = foxglove::schemas::PointCloud::from(msg);

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
pub(crate) struct OptimizedPointCloudChannel(Arc<Channel>);

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
            .build()
            .map_err(PyFoxgloveError::from)?;

        Ok(OptimizedPointCloudChannel(channel))
    }

    #[pyo3(signature = (msg, publish_time=None, log_time=None, sequence=None))]
    fn log<'py>(
        &self,
        // see https://pyo3.rs/v0.23.4/class.html#no-generic-parameters -- need to generate
        // concrete classes for each schema
        msg: Bound<'py, crate::py_schemas::OptimizedPointCloud>,
        publish_time: Option<u64>,
        log_time: Option<u64>,
        sequence: Option<u32>,
    ) -> PyResult<()> {
        let bytes_ref = msg.borrow();
        let slice = bytes_ref.data.as_bytes(msg.py());

        // Safety: cast slice from &[u8] to static.
        // This is not safe if log_with_meta or we keep a copy of the Bytes object, but we don't.
        // It's only valid for the duration of this function.
        let bytes = unsafe { bytes::Bytes::from_static(std::mem::transmute(slice)) };

        let point_cloud = foxglove::schemas::PointCloud {
            timestamp: None,
            frame_id: "".to_string(),
            pose: None,
            point_stride: 0,
            fields: vec![],
            data: bytes,
        };

        let mut buf = Vec::new();
        point_cloud
            .encode(&mut buf)
            .expect("Failed to encode SceneUpdate");

        let metadata = PartialMetadata {
            sequence,
            log_time,
            publish_time,
        };
        self.0.log_with_meta(&buf, metadata);

        Ok(())
    }
}

// testing out a bytearray directly.
// I think the copy here can be avoided by having our Prost types take `Bytes`
#[pyfunction]
pub(crate) fn log_bytes<'py>(bytes: Bound<'py, PyBytes>) -> PyResult<()> {
    println!("[rust] bytes: {:?}", bytes);

    let point_cloud = foxglove::schemas::PointCloud {
        timestamp: None,
        frame_id: "".to_string(),
        pose: None,
        point_stride: 0,
        fields: vec![],
        // extract copies data
        // data: bytes.extract()?,
        // as_bytes doesn't copy (I don't think), but to_vec does
        // https://docs.rs/pyo3/latest/pyo3/types/trait.PyBytesMethods.html
        // data: bytes.as_bytes().to_vec(),
        //
        // ...after prost takes byte references:
        //
        // does not live long enough:
        // data: bytes::Bytes::from_owner(bytes.as_bytes()),
        // escapes outside of function:
        // data: bytes::Bytes::from(bytes.as_bytes()),
        // data: bytes::Bytes::from_owner(b),
        // still copies
        data: bytes::Bytes::from(bytes.as_bytes().to_vec()),
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
    println!("pointCloud: {:?}", point_cloud);

    // works, but extract does a copy...
    let point_cloud = point_cloud.extract::<crate::py_schemas::PointCloud>()?;

    let update = foxglove::schemas::PointCloud::from(point_cloud);

    for item in list.into_iter() {
        println!("item: {:?}", item);
    }

    Ok(())
}
