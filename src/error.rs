use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NihError {
    DuplicatePluginId(String),
    MissingDependency {
        plugin_id: String,
        depends_on: String,
    },
    CircularDependency(String),
    ValidationFailed {
        plugin_id: String,
        reason: String,
    },
    ExecutionFailed {
        plugin_id: String,
        reason: String,
    },
}

impl Display for NihError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            NihError::DuplicatePluginId(id) => write!(f, "duplicate plugin id: {id}"),
            NihError::MissingDependency {
                plugin_id,
                depends_on,
            } => write!(
                f,
                "plugin '{plugin_id}' depends on missing plugin '{depends_on}'"
            ),
            NihError::CircularDependency(path) => write!(f, "circular dependency detected: {path}"),
            NihError::ValidationFailed { plugin_id, reason } => {
                write!(f, "validation failed in '{plugin_id}': {reason}")
            }
            NihError::ExecutionFailed { plugin_id, reason } => {
                write!(f, "execution failed in '{plugin_id}': {reason}")
            }
        }
    }
}

impl std::error::Error for NihError {}

pub type NihResult<T> = Result<T, NihError>;
