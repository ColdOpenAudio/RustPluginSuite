use crate::{NihResult, PluginContext};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PluginMetadata {
    pub id: &'static str,
    pub version: &'static str,
    pub dependencies: Vec<&'static str>,
    pub description: &'static str,
}

impl PluginMetadata {
    pub fn new(id: &'static str) -> Self {
        Self {
            id,
            version: "0.1.0",
            dependencies: Vec::new(),
            description: "",
        }
    }

    pub fn with_version(mut self, version: &'static str) -> Self {
        self.version = version;
        self
    }

    pub fn with_dependencies(mut self, deps: Vec<&'static str>) -> Self {
        self.dependencies = deps;
        self
    }

    pub fn with_description(mut self, description: &'static str) -> Self {
        self.description = description;
        self
    }
}

pub trait NihPlugin: Send + Sync {
    fn id(&self) -> &'static str;

    fn metadata(&self) -> PluginMetadata {
        PluginMetadata::new(self.id())
    }

    fn validate(&self, _ctx: &PluginContext) -> NihResult<()> {
        Ok(())
    }

    fn run(&self, ctx: &mut PluginContext) -> NihResult<()>;
}
