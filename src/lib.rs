use anyhow::Result;
use mesh_llm_plugin::{
    PluginMetadata, PluginRuntime, PluginStartupPolicy, capability, plugin_server_info,
};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
const PLUGIN_ID: &str = "metrics";
const TELEMETRY_CAPABILITY: &str = "telemetry.metrics.v1";

fn build_plugin(name: String) -> mesh_llm_plugin::SimplePlugin {
    mesh_llm_plugin::plugin! {
        metadata: PluginMetadata::new(
            name,
            VERSION,
            plugin_server_info(
                "mesh-metrics",
                VERSION,
                "Metrics Plugin",
                "Advertises mesh-llm metrics support through the plugin API.",
                Some(
                    "Configure mesh-llm [telemetry] settings for OTLP export. \
                     The plugin is decoupled from host internals and only uses \
                     the mesh-llm plugin API.",
                ),
            ),
        ),
        startup_policy: PluginStartupPolicy::Any,
        provides: [
            capability(TELEMETRY_CAPABILITY),
        ],
        health: |_context| {
            Box::pin(async move { Ok("metrics=plugin-api".to_string()) })
        },
    }
}

async fn run_plugin(name: String) -> Result<()> {
    PluginRuntime::run(build_plugin(name)).await
}

pub fn run_main() -> i32 {
    let mut builder = tokio::runtime::Builder::new_multi_thread();
    builder.enable_all();
    let runtime = builder.build().expect("build tokio runtime");
    runtime.block_on(async move {
        match run_plugin(PLUGIN_ID.to_string()).await {
            Ok(()) => 0,
            Err(err) => {
                eprintln!("{err:#}");
                tokio::time::sleep(std::time::Duration::from_millis(50)).await;
                1
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use mesh_llm_plugin::Plugin;

    use super::*;

    #[test]
    fn manifest_declares_metrics_capability() {
        let plugin = build_plugin(PLUGIN_ID.to_string());
        let manifest = plugin.manifest().expect("manifest");

        assert!(
            manifest
                .capabilities
                .iter()
                .any(|capability| capability == TELEMETRY_CAPABILITY)
        );
        assert!(manifest.endpoints.is_empty());
    }
}
