use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Accumulates planner warnings and provides helper methods for mutation.
#[derive(Debug, Clone, Default, Serialize, Deserialize, ToSchema)]
#[serde(transparent)]
pub struct Warnings(Vec<String>);

impl Warnings {
    /// Creates an empty warning collection.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds one warning message.
    pub fn add(&mut self, warning: impl Into<String>) {
        self.0.push(warning.into());
    }

    /// Adds an optional warning message when present.
    pub fn add_optional(&mut self, warning: Option<String>) {
        if let Some(warning) = warning {
            self.add(warning);
        }
    }

    /// Number of collected warning messages.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` when no warnings have been collected.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns collected warnings as a slice.
    pub fn as_slice(&self) -> &[String] {
        &self.0
    }

    /// Consumes the collection and returns the underlying vector.
    pub fn into_vec(self) -> Vec<String> {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::Warnings;

    #[test]
    fn warnings_new_and_add() {
        let mut warnings = Warnings::new();
        warnings.add("first warning");
        warnings.add_optional(Some("second warning".into()));
        warnings.add_optional(None);

        assert_eq!(warnings.len(), 2);
        assert_eq!(warnings.as_slice()[0], "first warning");
        assert_eq!(warnings.as_slice()[1], "second warning");
    }
}
