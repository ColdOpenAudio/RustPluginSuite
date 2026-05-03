use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PluginContext {
    data: HashMap<String, String>,
}

impl PluginContext {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.data.insert(key.into(), value.into());
    }

    pub fn get_string(&self, key: &str) -> Option<&str> {
        self.data.get(key).map(String::as_str)
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.data.contains_key(key)
    }

    pub fn remove(&mut self, key: &str) -> Option<String> {
        self.data.remove(key)
    }

    pub fn into_map(self) -> HashMap<String, String> {
        self.data
    }
}

impl Default for PluginContext {
    fn default() -> Self {
        Self::new()
    }
}
