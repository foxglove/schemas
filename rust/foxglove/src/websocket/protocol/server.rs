use crate::channel::Channel;
use crate::channel::ChannelId;
use crate::websocket::service::CallId;
use crate::websocket::service::ServiceId;
use crate::websocket::service::{self, Service};
use crate::FoxgloveError;
use base64::prelude::*;
use bytes::{BufMut, Bytes, BytesMut};
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_repr::Serialize_repr;
use serde_with::{base64::Base64, serde_as};
use std::collections::{HashMap, HashSet};

#[repr(u8)]
pub enum BinaryOpcode {
    MessageData = 1,
    #[cfg(feature = "unstable")]
    TimeData = 2,
    ServiceCallResponse = 3,
    // FetchAssetResponse = 4,
    // ServiceCallResponse = 3,
    FetchAssetResponse = 4,
}

#[derive(Debug, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Advertisement<'a> {
    pub id: ChannelId,
    pub topic: &'a str,
    pub encoding: &'a str,
    pub schema_name: &'a str,
    pub schema: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_encoding: Option<&'a str>,
}

/// A parameter type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ParameterType {
    /// A byte array, encoded as a base64-encoded string.
    ByteArray,
    /// A decimal or integer value that can be represented as a `float64`.
    Float64,
    /// An array of decimal or integer values that can be represented as `float64`s.
    Float64Array,
}

/// A parameter value.
#[serde_as]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ParameterValue {
    /// A decimal or integer value.
    Number(f64),
    /// A boolean value.
    Bool(bool),
    /// A byte array, encoded as a base64-encoded string.
    String(#[serde_as(as = "Base64")] Vec<u8>),
    /// An array of parameter values.
    Array(Vec<ParameterValue>),
    /// An associative map of parameter values.
    Dict(HashMap<String, ParameterValue>),
}

/// Informs the client about a parameter.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Parameter {
    /// The name of the parameter.
    pub name: String,
    /// The parameter type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<ParameterType>,
    /// The parameter value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<ParameterValue>,
}

#[derive(Serialize)]
#[serde(tag = "op")]
#[serde(rename_all = "camelCase")]
#[serde(rename_all_fields = "camelCase")]
pub enum ServerMessage<'a> {
    ParameterValues {
        #[serde(skip_serializing_if = "Option::is_none")]
        id: Option<&'a str>,
        parameters: &'a Vec<Parameter>,
    },
}

/// The log level for a [`Status`] message.
#[derive(Debug, Copy, Clone, Serialize_repr)]
#[repr(u8)]
#[allow(missing_docs)]
pub enum StatusLevel {
    Info = 0,
    Warning = 1,
    Error = 2,
}

/// A status message.
///
/// For more information, refer to the [Status][status] message specification.
///
/// [status]: https://github.com/foxglove/ws-protocol/blob/main/docs/spec.md#status
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "op")]
#[serde(rename = "status")]
pub struct Status {
    pub(crate) level: StatusLevel,
    pub(crate) message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) id: Option<String>,
}

impl Status {
    /// Creates a new status message.
    pub fn new(level: StatusLevel, message: String) -> Self {
        Self {
            level,
            message,
            id: None,
        }
    }

    /// Sets the status message ID, so that this status can be replaced or removed in the future.
    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }
}

#[derive(Serialize)]
#[serde(tag = "op")]
#[serde(rename = "removeStatus")]
#[serde(rename_all = "camelCase")]
pub struct RemoveStatus {
    pub status_ids: Vec<String>,
}

/// A capability that the websocket server advertises to its clients.
#[derive(Debug, Serialize, Eq, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub enum Capability {
    /// Allow clients to advertise channels to send data messages to the server.
    ClientPublish,
    /// Allow clients to get & set parameters.
    Parameters,
    /// Allow clients to subscribe and unsubscribe from parameter updates
    ParametersSubscribe,
    ///
    /// Inform clients about the latest server time.
    ///
    /// This allows accelerated, slowed, or stepped control over the progress of time. If the
    /// server publishes time data, then timestamps of published messages must originate from the
    /// same time source.
    #[cfg(feature = "unstable")]
    Time,
    /// Allow clients to call services.
    Services,
    /// Allow clients to request assets.
    Assets,
}

// https://github.com/foxglove/ws-protocol/blob/main/docs/spec.md#server-info
pub fn server_info(
    session_id: &str,
    name: &str,
    capabilities: &HashSet<Capability>,
    supported_encodings: &HashSet<String>,
) -> String {
    json!({
        "op": "serverInfo",
        "name": name,
        "capabilities": capabilities,
        "supportedEncodings": supported_encodings,
        "metadata": {},
        "sessionId": session_id
    })
    .to_string()
}

// https://github.com/foxglove/ws-protocol/blob/main/docs/spec.md#advertise
// Caller must check that the channel has a schema, otherwise this will panic.
pub fn advertisement(channel: &Channel) -> Result<String, FoxgloveError> {
    let schema = &channel
        .schema
        .as_ref()
        .ok_or_else(|| FoxgloveError::SchemaRequired)?;

    let schema_data = match schema.encoding.as_str() {
        "protobuf" => BASE64_STANDARD.encode(&schema.data),
        _ => String::from_utf8(schema.data.to_vec())
            .map_err(|e| FoxgloveError::Unspecified(e.into()))?,
    };

    Ok(json!({
        "op": "advertise",
        "channels": [Advertisement{
            id: channel.id,
            topic: &channel.topic,
            encoding: &channel.message_encoding,
            schema_name: &schema.name,
            schema: schema_data,
            schema_encoding: Some(&schema.encoding),
        }],
    })
    .to_string())
}

// https://github.com/foxglove/ws-protocol/blob/main/docs/spec.md#unadvertise
pub fn unadvertise(channel_id: ChannelId) -> String {
    json!({
        "op": "unadvertise",
        "channels": [channel_id],
    })
    .to_string()
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct AdvertiseService<'a> {
    id: ServiceId,
    name: &'a str,
    r#type: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    request: Option<AdvertiseServiceMessageSchema<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_schema: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    response: Option<AdvertiseServiceMessageSchema<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    response_schema: Option<&'a str>,
}

impl<'a> From<&'a Service> for AdvertiseService<'a> {
    fn from(service: &'a Service) -> Self {
        let schema = service.schema();
        let request = schema.request();
        let response = schema.response();
        Self {
            id: service.id(),
            name: service.name(),
            r#type: schema.name(),
            request: request.map(|r| r.into()),
            request_schema: if request.is_none() { Some("") } else { None },
            response: response.map(|r| r.into()),
            response_schema: if response.is_none() { Some("") } else { None },
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct AdvertiseServiceMessageSchema<'a> {
    encoding: &'a str,
    schema_name: &'a str,
    schema_encoding: &'a str,
    schema: &'a [u8],
}

impl<'a> From<&'a service::MessageSchema> for AdvertiseServiceMessageSchema<'a> {
    fn from(ms: &'a service::MessageSchema) -> Self {
        let schema = &ms.schema;
        Self {
            encoding: &ms.encoding,
            schema_name: &schema.name,
            schema_encoding: &schema.encoding,
            schema: &schema.data,
        }
    }
}

// https://github.com/foxglove/ws-protocol/blob/main/docs/spec.md#advertise-services
pub(crate) fn advertise_services<'a>(services: impl IntoIterator<Item = &'a Service>) -> String {
    let services: Vec<_> = services
        .into_iter()
        .map(|s| json!(AdvertiseService::from(s)))
        .collect();
    json!({
        "op": "advertiseServices",
        "services": services,
    })
    .to_string()
}

// https://github.com/foxglove/ws-protocol/blob/main/docs/spec.md#unadvertise-services
pub(crate) fn unadvertise_services(ids: &[ServiceId]) -> String {
    json!({
        "op": "unadvertiseServices",
        "serviceIds": ids,
    })
    .to_string()
}

pub fn parameters_json(parameters: &Vec<Parameter>, id: Option<&str>) -> String {
    // Serialize the parameters to JSON. This shouldn't fail, see serde_json::to_string docs.
    serde_json::to_string(&ServerMessage::ParameterValues { parameters, id })
        .expect("Failed to serialize parameters")
}

// https://github.com/foxglove/ws-protocol/blob/main/docs/spec.md#service-call-response
pub(crate) struct ServiceCallResponse {
    pub service_id: ServiceId,
    pub call_id: CallId,
    pub encoding: String,
    pub payload: Bytes,
}

impl ServiceCallResponse {
    pub fn new(service_id: ServiceId, call_id: CallId, encoding: String, payload: Bytes) -> Self {
        Self {
            service_id,
            call_id,
            encoding,
            payload,
        }
    }

    pub fn encode(self) -> Bytes {
        let encoding_raw = self.encoding.as_bytes();
        let mut buf = BytesMut::with_capacity(13 + encoding_raw.len() + self.payload.len());
        buf.put_u8(BinaryOpcode::ServiceCallResponse as u8);
        buf.put_u32_le(self.service_id.into());
        buf.put_u32_le(self.call_id.into());
        buf.put_u32_le(encoding_raw.len() as u32);
        buf.put(encoding_raw);
        buf.put(self.payload);
        buf.into()
    }
}

// https://github.com/foxglove/ws-protocol/blob/main/docs/spec.md#service-call-failure
pub(crate) fn service_call_failure(
    service_id: ServiceId,
    call_id: CallId,
    message: &str,
) -> String {
    json!({
        "op": "serviceCallFailure",
        "serviceId": service_id,
        "callId": call_id,
        "message": message,
    })
    .to_string()
}

#[cfg(test)]
mod tests {
    use service::ServiceSchema;

    use crate::Schema;

    use super::*;

    #[test]
    fn test_server_info() {
        let default = server_info("id:123", "name:test", &HashSet::new(), &HashSet::new());
        let expected = json!({
            "op": "serverInfo",
            "name": "name:test",
            "sessionId": "id:123",
            "capabilities": [],
            "supportedEncodings": [],
            "metadata": {},
        });
        assert_eq!(default, expected.to_string());

        let with_publish = server_info(
            "id:123",
            "name:test",
            &HashSet::from([Capability::ClientPublish]),
            &HashSet::from(["json".to_string()]),
        );
        let expected = json!({
            "op": "serverInfo",
            "name": "name:test",
            "sessionId": "id:123",
            "capabilities": ["clientPublish"],
            "supportedEncodings": ["json"],
            "metadata": {},
        });
        assert_eq!(with_publish, expected.to_string());
    }

    #[test]
    fn test_status() {
        fn json(level: StatusLevel) -> serde_json::Value {
            let status = Status {
                level,
                message: "test".to_string(),
                id: None,
            };
            serde_json::to_value(&status).expect("Failed to serialize status")
        }

        let info_json = json(StatusLevel::Info);
        assert_eq!(
            info_json,
            json!({
                "op": "status",
                "level": 0,
                "message": "test",
            })
        );

        let warning_json = json(StatusLevel::Warning);
        assert_eq!(
            warning_json,
            json!({
                "op": "status",
                "level": 1,
                "message": "test",
            })
        );

        let error_json = json(StatusLevel::Error);
        assert_eq!(
            error_json,
            json!({
                "op": "status",
                "level": 2,
                "message": "test",
            })
        );
    }

    #[test]
    fn test_parameter_values_byte_array() {
        let float_param = Parameter {
            name: "f64".to_string(),
            value: Some(ParameterValue::Number(1.23)),
            r#type: Some(ParameterType::Float64),
        };
        let float_array_param = Parameter {
            name: "f64[]".to_string(),
            value: Some(ParameterValue::Array(vec![
                ParameterValue::Number(1.23),
                ParameterValue::Number(4.56),
            ])),
            r#type: Some(ParameterType::Float64Array),
        };
        let data = vec![0x10, 0x20, 0x30];
        let byte_array_param = Parameter {
            name: "byte[]".to_string(),
            value: Some(ParameterValue::String(data.clone())),
            r#type: Some(ParameterType::ByteArray),
        };
        let bool_param = Parameter {
            name: "bool".to_string(),
            value: Some(ParameterValue::Bool(true)),
            r#type: None,
        };

        let parameters = vec![float_param, float_array_param, byte_array_param, bool_param];
        let result = parameters_json(&parameters, None);
        assert_eq!(
            result,
            json!({
                "op": "parameterValues",
                "parameters": [
                    {
                        "name": "f64",
                        "value": 1.23,
                        "type": "float64",
                    },
                    {
                        "name": "f64[]",
                        "type": "float64_array",
                        "value": [1.23, 4.56],
                    },
                    {
                        "name": "byte[]",
                        "type": "byte_array",
                        "value": BASE64_STANDARD.encode(data),
                    },
                    {
                        "name": "bool",
                        "value": true,
                    },
                ]
            })
            .to_string()
        );
    }

    #[test]
    fn test_nested_named_parameter_values() {
        let inner_value = ParameterValue::Dict(HashMap::from([(
            "inner".to_string(),
            ParameterValue::Number(1.0),
        )]));
        let outer = Parameter {
            name: "outer".to_string(),
            value: Some(ParameterValue::Dict(HashMap::from([(
                "wrapping".to_string(),
                inner_value,
            )]))),
            r#type: None,
        };
        let parameters = vec![outer];
        let result = parameters_json(&parameters, None);
        assert_eq!(
            result,
            json!({
                "op": "parameterValues",
                "parameters": [
                    {
                        "name": "outer",
                        "value": {
                            "wrapping": {
                                "inner": 1.0
                            }
                        }
                    }
                ]
            })
            .to_string()
        );
    }

    #[test]
    fn test_parameter_values_omitting_nulls() {
        let parameters = vec![Parameter {
            name: "test".to_string(),
            value: None,
            r#type: None,
        }];
        let result = parameters_json(&parameters, None);
        assert_eq!(
            result,
            json!({
                "op": "parameterValues",
                "parameters": [
                    {
                        "name": "test"
                    }
                ]
            })
            .to_string()
        );
    }

    #[test]
    fn test_advertise_services() {
        let s1_schema = ServiceSchema::new("std_srvs/Empty");
        let s1 = Service::builder("foo", s1_schema)
            .with_id(ServiceId::new(1))
            .sync_handler_fn(|_, _| Err("not implemented"));

        let s2_schema = ServiceSchema::new("std_srvs/SetBool")
            .with_request(
                "ros1",
                Schema::new("std_srvs/SetBool_Request", "ros1msg", b"bool data"),
            )
            .with_response(
                "ros1",
                Schema::new(
                    "std_srvs/SetBool_Response",
                    "ros1msg",
                    b"bool success\nstring message",
                ),
            );
        let s2 = Service::builder("set_bool", s2_schema)
            .with_id(ServiceId::new(2))
            .sync_handler_fn(|_, _| Err("not implemented"));

        let adv = advertise_services(&[s1, s2]);
        assert_eq!(
            adv,
            json!({
                "op": "advertiseServices",
                "services": [
                    {
                        "id": 1,
                        "name": "foo",
                        "type": "std_srvs/Empty",
                        "requestSchema": "",
                        "responseSchema": ""
                    },
                    {
                        "id": 2,
                        "name": "set_bool",
                        "type": "std_srvs/SetBool",
                        "request": {
                            "encoding": "ros1",
                            "schemaName": "std_srvs/SetBool_Request",
                            "schemaEncoding": "ros1msg",
                            "schema": b"bool data"
                        },
                        "response": {
                            "encoding": "ros1",
                            "schemaName": "std_srvs/SetBool_Response",
                            "schemaEncoding": "ros1msg",
                            "schema": b"bool success\nstring message"
                        }
                    }
                ]
            })
            .to_string()
        );
    }

    #[test]
    fn test_unadvertise_services() {
        let adv = unadvertise_services(&[ServiceId::new(1), ServiceId::new(2)]);
        assert_eq!(
            adv,
            json!({
                "op": "unadvertiseServices",
                "serviceIds": [1, 2],
            })
            .to_string()
        );
    }

    #[test]
    fn test_service_call_request() {
        let msg = ServiceCallResponse::new(
            ServiceId::new(1),
            CallId::new(2),
            "raw".to_string(),
            Bytes::from_static(b"yolo"),
        )
        .encode();
        let mut buf = BytesMut::new();
        buf.put_u8(BinaryOpcode::ServiceCallResponse as u8);
        buf.put_u32_le(1); // service id
        buf.put_u32_le(2); // call id
        buf.put_u32_le(3); // encoding length
        buf.put(b"raw".as_slice());
        buf.put(b"yolo".as_slice());
        assert_eq!(msg, buf);
    }

    #[test]
    fn test_service_call_failure() {
        let msg = service_call_failure(ServiceId::new(42), CallId::new(271828), "drat");
        assert_eq!(
            msg,
            json!({
                "op": "serviceCallFailure",
                "serviceId": 42,
                "callId": 271828,
                "message": "drat",
            })
            .to_string()
        );
    }
}
