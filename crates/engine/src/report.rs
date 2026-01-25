use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum Severity {
    Info,
    Warning,
    Error,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Issue {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub severity: Severity,
    pub source: Option<String>,
}

#[derive(Debug, Default)]
pub struct AnalysisReport {
    pub issues: Vec<Issue>,
}

impl AnalysisReport {
    pub fn merge(&mut self, other: AnalysisReport) {
        self.issues.extend(other.issues);
    }
}