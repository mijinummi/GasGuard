use crate::report::Issue;
use serde_json::Value;
use std::fs;
use std::path::PathBuf;

/// Minimal representation of a loaded contract.
/// For Soroban-first design, metadata can hold Soroban-specific info extracted later.
#[derive(Debug, Clone)]
pub struct Contract {
    pub path: PathBuf,
    pub bytes: Vec<u8>,
    pub metadata: Option<Value>,
}

impl Contract {
    pub fn load(path: impl Into<PathBuf>) -> anyhow::Result<Self> {
        let path = path.into();
        let bytes = fs::read(&path)?;
        let metadata = None;
        Ok(Self {
            path,
            bytes,
            metadata,
        })
    }
}

pub fn parsing_issue(id: impl Into<String>, title: impl Into<String>, desc: Option<String>) -> Issue {
    Issue {
        id: id.into(),
        title: title.into(),
        description: desc,
        severity: crate::report::Severity::Error,
        source: Some("parser".to_string()),
    }
}