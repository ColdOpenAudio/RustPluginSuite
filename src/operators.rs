use crate::{NihError, NihResult, PluginContext, PluginMetadata};

/// A sub operator represents one concrete plugin/VST unit under a shared suite.
///
/// Operators can publish their own VST metadata while still sharing a common
/// orchestration and context API.
pub trait SubOperator: Send + Sync {
    fn key(&self) -> &'static str;
    fn display_name(&self) -> &'static str;
    fn category(&self) -> &'static str;

    fn plugin_metadata(&self) -> PluginMetadata {
        PluginMetadata::new(self.key())
            .with_version("0.1.0")
            .with_description(self.display_name())
    }

    fn vst_descriptor(&self) -> VstDescriptor {
        VstDescriptor::new(self.key(), self.display_name(), self.category())
    }

    fn validate(&self, _ctx: &PluginContext) -> NihResult<()> {
        Ok(())
    }

    fn process(&self, ctx: &mut PluginContext) -> NihResult<()>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VstDescriptor {
    pub plugin_id: &'static str,
    pub name: &'static str,
    pub category: &'static str,
    pub vendor: &'static str,
    pub version: &'static str,
}

impl VstDescriptor {
    pub fn new(plugin_id: &'static str, name: &'static str, category: &'static str) -> Self {
        Self {
            plugin_id,
            name,
            category,
            vendor: "RustPluginSuite",
            version: "0.1.0",
        }
    }
}

#[derive(Default)]
pub struct OperatorSuite {
    operators: Vec<Box<dyn SubOperator>>,
}

impl OperatorSuite {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register<O: SubOperator + 'static>(&mut self, operator: O) -> NihResult<()> {
        if self.operators.iter().any(|o| o.key() == operator.key()) {
            return Err(NihError::DuplicatePluginId(operator.key().to_string()));
        }
        self.operators.push(Box::new(operator));
        Ok(())
    }

    pub fn descriptors(&self) -> Vec<VstDescriptor> {
        self.operators.iter().map(|o| o.vst_descriptor()).collect()
    }

    pub fn run_all(&self, ctx: &mut PluginContext) -> NihResult<()> {
        for operator in &self.operators {
            operator.validate(ctx)?;
            operator.process(ctx)?;
        }
        Ok(())
    }
}

#[derive(Default)]
pub struct OscilloscopeOperator;

impl SubOperator for OscilloscopeOperator {
    fn key(&self) -> &'static str {
        "oscilloscope"
    }

    fn display_name(&self) -> &'static str {
        "Oscilloscope"
    }

    fn category(&self) -> &'static str {
        "Analyzer"
    }

    fn process(&self, ctx: &mut PluginContext) -> NihResult<()> {
        ctx.set_subkey("operators.oscilloscope", "enabled", "true");
        Ok(())
    }
}

#[derive(Default)]
pub struct StereoscopeOperator;

impl SubOperator for StereoscopeOperator {
    fn key(&self) -> &'static str {
        "stereoscope"
    }

    fn display_name(&self) -> &'static str {
        "Stereoscope"
    }

    fn category(&self) -> &'static str {
        "Analyzer"
    }

    fn process(&self, ctx: &mut PluginContext) -> NihResult<()> {
        ctx.set_subkey("operators.stereoscope", "enabled", "true");
        Ok(())
    }
}

#[derive(Default)]
pub struct FrequencyGateOperator;

impl SubOperator for FrequencyGateOperator {
    fn key(&self) -> &'static str {
        "frequency_gate"
    }

    fn display_name(&self) -> &'static str {
        "Frequency Gate"
    }

    fn category(&self) -> &'static str {
        "Dynamics"
    }

    fn process(&self, ctx: &mut PluginContext) -> NihResult<()> {
        ctx.set_subkey("operators.frequency_gate", "enabled", "true");
        Ok(())
    }
}

#[derive(Default)]
pub struct BassGoBrrrOperator;

impl SubOperator for BassGoBrrrOperator {
    fn key(&self) -> &'static str {
        "bass_go_brrr"
    }

    fn display_name(&self) -> &'static str {
        "Bass Go Brrr"
    }

    fn category(&self) -> &'static str {
        "Bass Enhancement"
    }

    fn process(&self, ctx: &mut PluginContext) -> NihResult<()> {
        ctx.set_subkey("operators.bass_go_brrr", "enabled", "true");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn suite_registers_distinct_operators_and_exports_descriptors() {
        let mut suite = OperatorSuite::new();
        suite.register(OscilloscopeOperator).unwrap();
        suite.register(StereoscopeOperator).unwrap();

        let descriptors = suite.descriptors();
        assert_eq!(descriptors.len(), 2);
        assert_eq!(descriptors[0].plugin_id, "oscilloscope");
        assert_eq!(descriptors[1].plugin_id, "stereoscope");
    }

    #[test]
    fn suite_rejects_duplicate_operators() {
        let mut suite = OperatorSuite::new();
        suite.register(OscilloscopeOperator).unwrap();

        let err = suite.register(OscilloscopeOperator).unwrap_err();
        assert!(matches!(err, NihError::DuplicatePluginId(_)));
    }

    #[test]
    fn suite_runs_all_operators_and_records_state() {
        let mut suite = OperatorSuite::new();
        suite.register(OscilloscopeOperator).unwrap();
        suite.register(StereoscopeOperator).unwrap();
        suite.register(FrequencyGateOperator).unwrap();
        suite.register(BassGoBrrrOperator).unwrap();

        let mut ctx = PluginContext::new();
        suite.run_all(&mut ctx).unwrap();

        assert_eq!(
            ctx.get_subkey("operators.oscilloscope", "enabled"),
            Some("true")
        );
        assert_eq!(
            ctx.get_subkey("operators.stereoscope", "enabled"),
            Some("true")
        );
        assert_eq!(
            ctx.get_subkey("operators.frequency_gate", "enabled"),
            Some("true")
        );
        assert_eq!(
            ctx.get_subkey("operators.bass_go_brrr", "enabled"),
            Some("true")
        );
    }
}
