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

## Compatibility

Version 0.1.1 requires a MeshLLM host that supports plugin protocol 3.
Protocol 2 hosts cannot load this version. The plugin SDK is pinned to MeshLLM
commit `efd30516fd2cd92ac132dfd7b8341d24d412ee9c` for reproducible builds.

## Build

```bash
cargo build --locked
```

## Release Archives

Push a `v*` tag from a validated commit to publish a release. To recover an
interrupted release, select the reviewed `main` branch in the `Release`
workflow and enter the existing version tag. The selected branch supplies
the workflow; builds check out `refs/tags/<version>`. Confirm that the tag
still points to the validated commit and no release assets have been published
before running it manually. macOS archives use the ARM64 `macos-15` runner.

GitHub releases package archives using the same contract as
`Mesh-LLM/blackboard`:

- `metrics-<target>.tar.gz` or `.zip`
- `metrics-<version>-<target>.tar.gz` or `.zip`

Each archive contains:

- `metrics/metrics`
- `metrics/plugin.toml`
