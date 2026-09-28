# metrics

`metrics` is an external mesh-llm plugin that advertises metrics support through
the mesh-llm plugin API.

Telemetry export is configured in mesh-llm with `[telemetry]`; this plugin does
not read mesh-llm internals or receive private telemetry payloads.

## Install

```bash
mesh-llm plugins install metrics
```

You can also install directly from GitHub:

```bash
mesh-llm plugins install Mesh-LLM/metrics
```

## Configure

Enable the plugin in `~/.mesh-llm/config.toml`:

```toml
[[plugin]]
name = "metrics"
```

Configure OTLP export with mesh-llm telemetry settings:

```toml
[telemetry]
enabled = true
endpoint = "https://otel.example.com"
```

## Build

```bash
cargo build
```

## Release Archives

GitHub releases package archives using the same contract as
`Mesh-LLM/blackboard`:

- `metrics-<target>.tar.gz` or `.zip`
- `metrics-<version>-<target>.tar.gz` or `.zip`

Each archive contains:

- `metrics/metrics`
- `metrics/plugin.toml`

## Compatibility

The plugin SDK is pinned to Mesh v0.77.0 (plugin protocol 3).
This build requires a protocol-3 host; older protocol-2 hosts must keep their
previous plugin release or upgrade Mesh together with the plugin.
