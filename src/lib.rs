//! # RustPluginSuite
//!
//! A plug-and-play **NIH framework** ("Not Invented Here") for building
//! composable plugin pipelines in Rust with strong typing, validation, and
//! execution lifecycle hooks.
//!
//! ## Quick start
//! ```
//! use rust_plugin_suite::{Framework, NihPlugin, NihResult, PluginContext};
//!
//! #[derive(Default)]
//! struct AddGreeting;
//!
//! impl NihPlugin for AddGreeting {
//!     fn id(&self) -> &'static str { "add_greeting" }
//!
//!     fn run(&self, ctx: &mut PluginContext) -> NihResult<()> {
//!         ctx.set("greeting", "hello");
//!         Ok(())
//!     }
//! }
//!
//! let mut framework = Framework::builder().build();
//! framework.register(AddGreeting::default()).unwrap();
//!
//! let mut ctx = PluginContext::new();
//! framework.execute(&mut ctx).unwrap();
//! assert_eq!(ctx.get_string("greeting"), Some("hello"));
//! ```

pub mod context;
pub mod error;
pub mod framework;
pub mod plugin;

pub use context::PluginContext;
pub use error::{NihError, NihResult};
pub use framework::{Framework, FrameworkBuilder, FrameworkReport};
pub use plugin::{NihPlugin, PluginMetadata};
