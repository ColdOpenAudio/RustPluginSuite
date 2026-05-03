# RustPluginSuite

A production-ready, plug-and-play **NIH framework** for Rust plugin orchestration.

> NIH here stands for **Not Invented Here**: you can drop in your own plugins, preserve full control, and avoid heavy external orchestration dependencies.

## Features

- Typed plugin interface with lifecycle hooks (`validate`, `run`)
- Dependency-aware execution with automatic topological ordering
- Explicit error model for duplicate IDs, missing deps, cycles, validation, and execution failures
- Configurable fail-fast behavior
- Test coverage for ordering, dependency validation, and fail-fast semantics

## Installation

```bash
cargo add rust_plugin_suite
```

(For local use in this repository, run `cargo build`.)

## Quick Start

```rust
use rust_plugin_suite::{Framework, NihPlugin, NihResult, PluginContext, PluginMetadata};

struct Bootstrap;
impl NihPlugin for Bootstrap {
    fn id(&self) -> &'static str { "bootstrap" }

    fn metadata(&self) -> PluginMetadata {
        PluginMetadata::new(self.id())
            .with_version("1.0.0")
            .with_description("Bootstraps app state")
    }

    fn run(&self, ctx: &mut PluginContext) -> NihResult<()> {
        ctx.set("app.ready", "true");
        Ok(())
    }
}

struct Feature;
impl NihPlugin for Feature {
    fn id(&self) -> &'static str { "feature" }

    fn metadata(&self) -> PluginMetadata {
        PluginMetadata::new(self.id())
            .with_dependencies(vec!["bootstrap"])
    }

    fn validate(&self, ctx: &PluginContext) -> NihResult<()> {
        if ctx.get_string("app.ready") != Some("true") {
            return Err(rust_plugin_suite::NihError::ValidationFailed {
                plugin_id: self.id().to_string(),
                reason: "app not ready".to_string(),
            });
        }
        Ok(())
    }

    fn run(&self, ctx: &mut PluginContext) -> NihResult<()> {
        ctx.set("feature.enabled", "true");
        Ok(())
    }
}

fn main() -> NihResult<()> {
    let mut framework = Framework::builder().fail_fast(true).build();
    framework.register(Bootstrap)?;
    framework.register(Feature)?;

    let mut ctx = PluginContext::new();
    let report = framework.execute(&mut ctx)?;

    assert_eq!(report.execution_order, vec!["bootstrap", "feature"]);
    assert_eq!(ctx.get_string("feature.enabled"), Some("true"));
    Ok(())
}
```

## Architecture

### Core types

- `NihPlugin`: Trait all plugins implement
- `PluginMetadata`: Plugin ID, version, dependencies, description
- `PluginContext`: Shared mutable key/value state (String-based)
- `Framework`: Registry + dependency resolver + executor
- `NihError`: Typed failure modes

### Execution flow

1. Register plugins by unique ID
2. Resolve dependency graph
3. Validate each plugin
4. Run each plugin in dependency order
5. Return `FrameworkReport` with execution order

## Development

```bash
cargo fmt
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```
