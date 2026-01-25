use crate::parser::Contract;
use crate::report::AnalysisReport;
use anyhow::Result;

/// Analyzer trait: a pluggable analysis unit that inspects a Contract and returns a report.
pub trait Analyzer: Send + Sync {
    fn name(&self) -> &'static str;
    fn analyze(&self, contract: &Contract) -> Result<AnalysisReport>;
}

/// A simple placeholder analyzer that produces no issues.
pub struct PlaceholderAnalyzer;

impl PlaceholderAnalyzer {
    pub fn new() -> Self {
        Self
    }
}

impl Analyzer for PlaceholderAnalyzer {
    fn name(&self) -> &'static str {
        "placeholder"
    }

    fn analyze(&self, _contract: &Contract) -> Result<AnalysisReport> {
        Ok(AnalysisReport::default())
    }
}