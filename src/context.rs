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

    pub fn set_subkey(
        &mut self,
        namespace: impl AsRef<str>,
        subkey: impl AsRef<str>,
        value: impl Into<String>,
    ) {
        self.set(
            Self::compose_subkey(namespace.as_ref(), subkey.as_ref()),
            value,
        );
    }

    pub fn get_string(&self, key: &str) -> Option<&str> {
        self.data.get(key).map(String::as_str)
    }

    pub fn get_subkey(&self, namespace: &str, subkey: &str) -> Option<&str> {
        self.get_string(&Self::compose_subkey(namespace, subkey))
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.data.contains_key(key)
    }

    pub fn contains_subkey(&self, namespace: &str, subkey: &str) -> bool {
        self.contains_key(&Self::compose_subkey(namespace, subkey))
    }

    pub fn remove(&mut self, key: &str) -> Option<String> {
        self.data.remove(key)
    }

    pub fn remove_subkey(&mut self, namespace: &str, subkey: &str) -> Option<String> {
        self.remove(&Self::compose_subkey(namespace, subkey))
    }

    pub fn list_subkeys<'a>(
        &'a self,
        namespace: &'a str,
    ) -> impl Iterator<Item = (&'a str, &'a str)> {
        let prefix = Self::namespace_prefix(namespace);
        self.data.iter().filter_map(move |(key, value)| {
            key.strip_prefix(&prefix)
                .map(|stripped_key| (stripped_key, value.as_str()))
        })
    }

    pub fn remove_namespace(&mut self, namespace: &str) -> Vec<(String, String)> {
        let prefix = Self::namespace_prefix(namespace);
        let keys_to_remove: Vec<String> = self
            .data
            .keys()
            .filter(|key| key.starts_with(&prefix))
            .cloned()
            .collect();

        keys_to_remove
            .into_iter()
            .filter_map(|key| self.remove_entry_by_key(&key))
            .collect()
    }

    pub fn into_map(self) -> HashMap<String, String> {
        self.data
    }

    fn remove_entry_by_key(&mut self, key: &str) -> Option<(String, String)> {
        self.data.remove_entry(key)
    }

    fn compose_subkey(namespace: &str, subkey: &str) -> String {
        format!(
            "{}.{}",
            namespace.trim_end_matches('.'),
            subkey.trim_start_matches('.')
        )
    }

    fn namespace_prefix(namespace: &str) -> String {
        format!("{}.", namespace.trim_end_matches('.'))
    }
}

impl Default for PluginContext {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::PluginContext;

    #[test]
    fn supports_subkey_crud() {
        let mut ctx = PluginContext::new();
        ctx.set_subkey("ui", "theme", "dark");

        assert_eq!(ctx.get_subkey("ui", "theme"), Some("dark"));
        assert!(ctx.contains_subkey("ui", "theme"));
        assert_eq!(ctx.remove_subkey("ui", "theme"), Some("dark".to_string()));
        assert_eq!(ctx.get_subkey("ui", "theme"), None);
    }

    #[test]
    fn lists_and_removes_namespace() {
        let mut ctx = PluginContext::new();
        ctx.set_subkey("audio", "gain", "0.7");
        ctx.set_subkey("audio", "pan", "0.2");
        ctx.set_subkey("ui", "theme", "light");

        let mut audio_keys: Vec<(&str, &str)> = ctx.list_subkeys("audio").collect();
        audio_keys.sort_unstable_by_key(|(k, _)| *k);

        assert_eq!(audio_keys, vec![("gain", "0.7"), ("pan", "0.2")]);

        let mut removed = ctx.remove_namespace("audio");
        removed.sort_unstable_by(|a, b| a.0.cmp(&b.0));

        assert_eq!(removed.len(), 2);
        assert!(!ctx.contains_subkey("audio", "gain"));
        assert!(!ctx.contains_subkey("audio", "pan"));
        assert_eq!(ctx.get_subkey("ui", "theme"), Some("light"));
    }
}
