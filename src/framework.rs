use std::collections::{HashMap, HashSet};

use crate::{NihError, NihPlugin, NihResult, PluginContext};

#[derive(Debug, Clone)]
pub struct FrameworkReport {
    pub execution_order: Vec<String>,
}

#[derive(Default)]
pub struct FrameworkBuilder {
    fail_fast: bool,
}

impl FrameworkBuilder {
    pub fn fail_fast(mut self, value: bool) -> Self {
        self.fail_fast = value;
        self
    }

    pub fn build(self) -> Framework {
        Framework {
            plugins: HashMap::new(),
            fail_fast: self.fail_fast,
        }
    }
}

pub struct Framework {
    plugins: HashMap<String, Box<dyn NihPlugin>>,
    fail_fast: bool,
}

impl Framework {
    pub fn builder() -> FrameworkBuilder {
        FrameworkBuilder::default()
    }

    pub fn register<P: NihPlugin + 'static>(&mut self, plugin: P) -> NihResult<()> {
        let id = plugin.id().to_string();
        if self.plugins.contains_key(&id) {
            return Err(NihError::DuplicatePluginId(id));
        }
        self.plugins.insert(id, Box::new(plugin));
        Ok(())
    }

    pub fn execute(&self, ctx: &mut PluginContext) -> NihResult<FrameworkReport> {
        let order = self.resolve_execution_order()?;
        let mut executed = Vec::with_capacity(order.len());

        for id in order {
            let plugin = self.plugins.get(&id).expect("plugin exists in map");

            if let Err(err) = plugin.validate(ctx) {
                if self.fail_fast {
                    return Err(err);
                }
                continue;
            }

            if let Err(err) = plugin.run(ctx) {
                if self.fail_fast {
                    return Err(err);
                }
                continue;
            }

            executed.push(id);
        }

        Ok(FrameworkReport {
            execution_order: executed,
        })
    }

    fn resolve_execution_order(&self) -> NihResult<Vec<String>> {
        let mut visiting = HashSet::<String>::new();
        let mut visited = HashSet::<String>::new();
        let mut result = Vec::<String>::new();

        for id in self.plugins.keys() {
            self.visit(
                id,
                &mut visiting,
                &mut visited,
                &mut result,
                &mut Vec::new(),
            )?;
        }

        Ok(result)
    }

    fn visit(
        &self,
        id: &str,
        visiting: &mut HashSet<String>,
        visited: &mut HashSet<String>,
        result: &mut Vec<String>,
        path: &mut Vec<String>,
    ) -> NihResult<()> {
        if visited.contains(id) {
            return Ok(());
        }

        if !self.plugins.contains_key(id)
            && let Some(origin) = path.last()
        {
            return Err(NihError::MissingDependency {
                plugin_id: origin.clone(),
                depends_on: id.to_string(),
            });
        }

        if visiting.contains(id) {
            let start = path.iter().position(|p| p == id).unwrap_or(0);
            let mut cycle = path[start..].to_vec();
            cycle.push(id.to_string());
            return Err(NihError::CircularDependency(cycle.join(" -> ")));
        }

        visiting.insert(id.to_string());
        path.push(id.to_string());

        let deps = self
            .plugins
            .get(id)
            .expect("plugin exists when visiting")
            .metadata()
            .dependencies;

        for dep in deps {
            if !self.plugins.contains_key(dep) {
                return Err(NihError::MissingDependency {
                    plugin_id: id.to_string(),
                    depends_on: dep.to_string(),
                });
            }
            self.visit(dep, visiting, visited, result, path)?;
        }

        path.pop();
        visiting.remove(id);
        visited.insert(id.to_string());
        result.push(id.to_string());

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{Framework, NihError, NihPlugin, NihResult, PluginContext, PluginMetadata};

    #[derive(Default)]
    struct SetPlugin {
        id: &'static str,
        key: &'static str,
        value: &'static str,
        deps: Vec<&'static str>,
    }

    impl NihPlugin for SetPlugin {
        fn id(&self) -> &'static str {
            self.id
        }

        fn metadata(&self) -> PluginMetadata {
            PluginMetadata::new(self.id).with_dependencies(self.deps.clone())
        }

        fn run(&self, ctx: &mut PluginContext) -> NihResult<()> {
            ctx.set(self.key, self.value);
            Ok(())
        }
    }

    struct InvalidPlugin;
    impl NihPlugin for InvalidPlugin {
        fn id(&self) -> &'static str {
            "invalid"
        }

        fn validate(&self, _: &PluginContext) -> NihResult<()> {
            Err(NihError::ValidationFailed {
                plugin_id: self.id().to_string(),
                reason: "forced".to_string(),
            })
        }

        fn run(&self, _ctx: &mut PluginContext) -> NihResult<()> {
            Ok(())
        }
    }

    #[test]
    fn executes_plugins_in_dependency_order() {
        let mut fw = Framework::builder().build();
        fw.register(SetPlugin {
            id: "a",
            key: "a",
            value: "1",
            deps: vec![],
        })
        .unwrap();
        fw.register(SetPlugin {
            id: "b",
            key: "b",
            value: "2",
            deps: vec!["a"],
        })
        .unwrap();

        let mut ctx = PluginContext::new();
        let report = fw.execute(&mut ctx).unwrap();

        assert_eq!(ctx.get_string("a"), Some("1"));
        assert_eq!(ctx.get_string("b"), Some("2"));
        assert_eq!(
            report.execution_order,
            vec!["a".to_string(), "b".to_string()]
        );
    }

    #[test]
    fn detects_missing_dependency() {
        let mut fw = Framework::builder().build();
        fw.register(SetPlugin {
            id: "b",
            key: "b",
            value: "2",
            deps: vec!["a"],
        })
        .unwrap();

        let err = fw.execute(&mut PluginContext::new()).unwrap_err();
        assert!(matches!(err, NihError::MissingDependency { .. }));
    }

    #[test]
    fn fail_fast_returns_validation_error() {
        let mut fw = Framework::builder().fail_fast(true).build();
        fw.register(InvalidPlugin).unwrap();

        let err = fw.execute(&mut PluginContext::new()).unwrap_err();
        assert!(matches!(err, NihError::ValidationFailed { .. }));
    }
}
