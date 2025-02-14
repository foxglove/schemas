//! Example of a parameter server using the Foxglove SDK.
//!
//! This example uses the 'unstable' feature to expose capabilities.
//!
//! Usage:
//! ```text
//! cargo run -p example-param-server
//! ```

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use clap::Parser;
use foxglove::websocket::{
    Capability, Client, Parameter, ParameterType, ParameterValue, ServerListener,
};
use foxglove::{WebSocketServer, WebSocketServerHandle};
use tokio_util::sync::CancellationToken;

#[derive(Debug, Parser)]
struct Cli {
    #[arg(short, long, default_value_t = 8765)]
    port: u16,
    #[arg(long, default_value = "127.0.0.1")]
    host: String,
}

struct ParamListener {
    params: Mutex<HashMap<String, Parameter>>,
    subscribed_params: Mutex<HashSet<String>>,
}

impl ParamListener {
    fn new() -> Arc<Self> {
        Arc::new(Self {
            params: Mutex::new(HashMap::new()),
            subscribed_params: Mutex::new(HashSet::new()),
        })
    }
}

impl ServerListener for ParamListener {
    fn on_get_parameters(
        &self,
        _client: Client,
        param_names: Vec<String>,
        request_id: Option<&str>,
    ) -> Vec<Parameter> {
        println!(
            "on_get_parameters called with parameter names: {:?}; request_id: {}",
            param_names,
            request_id.unwrap_or("None")
        );
        let params = self.params.lock().unwrap();
        param_names
            .iter()
            .filter_map(|name| params.get(name).cloned())
            .collect()
    }
    fn on_set_parameters(
        &self,
        _client: Client,
        parameters: Vec<Parameter>,
        request_id: Option<&str>,
    ) -> Vec<Parameter> {
        let param_names: Vec<String> = parameters.iter().map(|param| param.name.clone()).collect();
        println!(
            "on_set_parameters called with parameter names: {:?}; request_id: {}",
            param_names,
            request_id.unwrap_or("None")
        );
        let mut params = self.params.lock().unwrap();
        for param in parameters.iter() {
            params.insert(param.name.clone(), param.clone());
        }
        parameters
    }
    fn on_parameters_subscribe(&self, param_names: Vec<String>) {
        println!(
            "on_parameters_subscribe called with parameter names: {:?}",
            param_names
        );
        let mut subscribed_params = self.subscribed_params.lock().unwrap();
        for param_name in param_names {
            subscribed_params.insert(param_name);
        }
    }
    fn on_parameters_unsubscribe(&self, param_names: Vec<String>) {
        println!(
            "on_parameters_unsubscribe called with parameter names: {:?}",
            param_names
        );
        let mut subscribed_params = self.subscribed_params.lock().unwrap();
        for param_name in param_names {
            subscribed_params.remove(&param_name);
        }
    }
}

#[tokio::main]
async fn main() {
    let env = env_logger::Env::default().default_filter_or("debug");
    env_logger::init_from_env(env);

    let args = Cli::parse();
    let listener = ParamListener::new();

    let server = WebSocketServer::new()
        .name("param server")
        .capabilities([Capability::Parameters])
        .listener(listener.clone())
        .bind(args.host, args.port)
        .start()
        .await
        .expect("Failed to start server");

    let shutdown = watch_ctrl_c();
    tokio::select! {
        () = shutdown.cancelled() => (),
        () = update_parameters(&server, listener) => (),
    };

    server.stop().await;
}

async fn update_parameters(server: &WebSocketServerHandle, _listener: Arc<ParamListener>) {
    let start = Instant::now();
    let mut interval = tokio::time::interval(Duration::from_secs(1));
    loop {
        interval.tick().await;
        let parameter = Parameter {
            name: "elapsed".to_string(),
            value: Some(ParameterValue::Number(start.elapsed().as_secs_f64())),
            r#type: Some(ParameterType::Float64),
        };
        server.publish_parameter_values(vec![parameter]);
    }
}

fn watch_ctrl_c() -> CancellationToken {
    let token = CancellationToken::new();
    tokio::spawn({
        let token = token.clone();
        async move {
            tokio::signal::ctrl_c().await.ok();
            token.cancel();
        }
    });
    token
}
