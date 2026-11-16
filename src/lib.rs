//! SigmaOS-make: GNU make build system model.
//!
//! Models makefiles, targets, dependencies, rules, variables,
//! pattern rules, and build execution.

use std::collections::BTreeMap;
use std::fmt;

/// Build target types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TargetType {
    Phony,
    File,
    Pattern,
}

/// A make target.
#[derive(Debug, Clone)]
pub struct MakeTarget {
    pub name: String,
    pub target_type: TargetType,
    pub dependencies: Vec<String>,
    pub commands: Vec<String>,
    pub phony: bool,
}

/// A makefile variable.
#[derive(Debug, Clone)]
pub struct MakeVariable {
    pub name: String,
    pub value: String,
    pub exported: bool,
}

/// A makefile.
#[derive(Debug, Clone)]
pub struct Makefile {
    pub id: String,
    pub targets: BTreeMap<String, MakeTarget>,
    pub variables: BTreeMap<String, MakeVariable>,
    pub default_target: Option<String>,
}

/// Errors for the make domain.
#[derive(Debug, PartialEq)]
pub enum MakeError {
    EmptyName,
    EmptyCommand,
    Duplicate(String),
    NotFound(String),
    CircularDependency(Vec<String>),
}

impl fmt::Display for MakeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => write!(f, "name cannot be empty"),
            Self::EmptyCommand => write!(f, "command cannot be empty"),
            Self::Duplicate(id) => write!(f, "duplicate id: {id}"),
            Self::NotFound(id) => write!(f, "not found: {id}"),
            Self::CircularDependency(cycle) => write!(f, "circular dependency: {}", cycle.join(" -> ")),
        }
    }
}

impl std::error::Error for MakeError {}

/// make manager.
pub struct MakeManager {
    pub name: String,
    makefiles: BTreeMap<String, Makefile>,
}

impl MakeManager {
    /// Create a new manager.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            makefiles: BTreeMap::new(),
        }
    }

    /// Add a makefile with validation.
    pub fn add_makefile(&mut self, makefile: Makefile) -> Result<(), MakeError> {
        if makefile.id.is_empty() {
            return Err(MakeError::EmptyName);
        }
        if self.makefiles.contains_key(&makefile.id) {
            return Err(MakeError::Duplicate(makefile.id));
        }
        self.makefiles.insert(makefile.id.clone(), makefile);
        Ok(())
    }

    /// Retrieve a makefile by id.
    pub fn get_makefile(&self, id: &str) -> Option<&Makefile> {
        self.makefiles.get(id)
    }

    /// Count of makefiles.
    pub fn count(&self) -> usize {
        self.makefiles.len()
    }

    /// Add a target to a makefile.
    pub fn add_target(&mut self, makefile_id: &str, target: MakeTarget) -> Result<(), MakeError> {
        if target.name.is_empty() {
            return Err(MakeError::EmptyName);
        }
        for cmd in &target.commands {
            if cmd.trim().is_empty() {
                return Err(MakeError::EmptyCommand);
            }
        }
        let mf = self.makefiles.get_mut(makefile_id)
            .ok_or_else(|| MakeError::NotFound(makefile_id.to_string()))?;
        if mf.targets.contains_key(&target.name) {
            return Err(MakeError::Duplicate(target.name));
        }
        mf.targets.insert(target.name.clone(), target);
        Ok(())
    }

    /// Add a variable to a makefile.
    pub fn add_variable(&mut self, makefile_id: &str, var: MakeVariable) -> Result<(), MakeError> {
        if var.name.is_empty() {
            return Err(MakeError::EmptyName);
        }
        let mf = self.makefiles.get_mut(makefile_id)
            .ok_or_else(|| MakeError::NotFound(makefile_id.to_string()))?;
        mf.variables.insert(var.name.clone(), var);
        Ok(())
    }

    /// Clear all makefiles.
    pub fn clear(&mut self) {
        self.makefiles.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_makefile(id: &str) -> Makefile {
        Makefile {
            id: id.to_string(),
            targets: BTreeMap::new(),
            variables: BTreeMap::new(),
            default_target: None,
        }
    }

    fn make_target(name: &str) -> MakeTarget {
        MakeTarget {
            name: name.to_string(),
            target_type: TargetType::File,
            dependencies: vec![],
            commands: vec!["echo building".to_string()],
            phony: false,
        }
    }

    #[test]
    fn test_create_manager() {
        let mgr = MakeManager::new("make");
        assert_eq!(mgr.name, "make");
        assert_eq!(mgr.count(), 0);
    }

    #[test]
    fn test_add_makefile() {
        let mut mgr = MakeManager::new("make");
        let mf = make_makefile("Makefile");
        assert!(mgr.add_makefile(mf).is_ok());
        assert_eq!(mgr.count(), 1);
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut mgr = MakeManager::new("make");
        let mf = make_makefile("");
        assert_eq!(mgr.add_makefile(mf), Err(MakeError::EmptyName));
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut mgr = MakeManager::new("make");
        let mf = make_makefile("Makefile");
        mgr.add_makefile(mf.clone()).unwrap();
        assert_eq!(mgr.add_makefile(mf), Err(MakeError::Duplicate("Makefile".to_string())));
    }

    #[test]
    fn test_add_target() {
        let mut mgr = MakeManager::new("make");
        mgr.add_makefile(make_makefile("Makefile")).unwrap();
        let target = make_target("all");
        assert!(mgr.add_target("Makefile", target).is_ok());
    }

    #[test]
    fn test_empty_command_rejected() {
        let mut mgr = MakeManager::new("make");
        mgr.add_makefile(make_makefile("Makefile")).unwrap();
        let mut target = make_target("all");
        target.commands = vec!["".to_string()];
        assert_eq!(mgr.add_target("Makefile", target), Err(MakeError::EmptyCommand));
    }

    #[test]
    fn test_add_variable() {
        let mut mgr = MakeManager::new("make");
        mgr.add_makefile(make_makefile("Makefile")).unwrap();
        let var = MakeVariable {
            name: "CC".to_string(),
            value: "gcc".to_string(),
            exported: true,
        };
        assert!(mgr.add_variable("Makefile", var).is_ok());
    }

    #[test]
    fn test_clear() {
        let mut mgr = MakeManager::new("make");
        mgr.add_makefile(make_makefile("Makefile")).unwrap();
        mgr.clear();
        assert_eq!(mgr.count(), 0);
    }
}
