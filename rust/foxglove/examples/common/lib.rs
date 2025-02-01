use std::borrow::Cow;
use std::time::Duration;

use foxglove::schemas::{
    Color, CubePrimitive, FrameTransform, FrameTransformChannel, Pose, Quaternion, SceneEntity,
    SceneUpdate, SceneUpdateChannel, Vector3,
};
use foxglove::Channel;
use foxglove_sdk_core::Schema;
use serde_json::json;

fn euler_to_quaternion(roll: f64, pitch: f64, yaw: f64) -> Quaternion {
    let quat = quaternion::euler_angles(roll, pitch, yaw);
    Quaternion {
        x: quat.1[0],
        y: quat.1[1],
        z: quat.1[2],
        w: quat.0,
    }
}

pub async fn log_forever(fps: u8) {
    let sin_chan = Channel::new(
        "/sine",
        "json",
        Schema::new(
            "example_schema".to_string(),
            Some("jsonschema".to_string()),
            // This should be an actual json schema
            Cow::Borrowed(br#"{"type":"object","properties":{"msg":{"type":"string"},"count":{"type":"number"}}}"#),
        ),
    )
    .expect("Failed to create /sine channel");

    let box_chan = SceneUpdateChannel::new("/boxes").expect("Failed to create /boxes channel");

    let tf_chan = FrameTransformChannel::new("/tf").expect("Failed to create /tf channel");

    let mut counter: u32 = 0;
    let mut interval = tokio::time::interval(Duration::from_millis(1000 / u64::from(fps)));
    loop {
        interval.tick().await;
        counter += 1;
        let payload = json!({
            "msg": "Hello!",
            "count": counter,
        })
        .to_string();

        sin_chan.log(payload.as_bytes());

        let scene_update = SceneUpdate {
            deletions: vec![],
            entities: vec![SceneEntity {
                frame_id: "box".to_string(),
                id: "box_1".to_string(),
                lifetime: Some(prost_types::Duration {
                    seconds: 10,
                    nanos: 0,
                }),
                cubes: vec![CubePrimitive {
                    pose: Some(Pose {
                        position: Some(Vector3 {
                            x: 0.0,
                            y: 0.0,
                            z: 3.0,
                        }),
                        orientation: Some(euler_to_quaternion(0.0, 0.0, f64::from(counter) * -0.1)),
                    }),
                    size: Some(Vector3 {
                        x: 1.0,
                        y: 1.0,
                        z: 1.0,
                    }),
                    color: Some(Color {
                        r: 1.0,
                        g: 0.0,
                        b: 0.0,
                        a: 1.0,
                    }),
                }],
                ..Default::default()
            }],
        };

        let tf = FrameTransform {
            parent_frame_id: "world".to_string(),
            child_frame_id: "box".to_string(),
            rotation: Some(euler_to_quaternion(1.0, 0.0, f64::from(counter) * 0.1)),
            ..Default::default()
        };

        box_chan.log(&scene_update);
        tf_chan.log(&tf);
    }
}
