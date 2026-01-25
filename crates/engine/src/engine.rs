use crate::analyzer::Analyzer;
use crate::parser::Contract;
use crate::report::AnalysisReport;
use anyhow::Result;
use std::sync::Arc;

/// Engine: owns analyzers and coordinates parsing + rule execution.
pub struct Engine {
    analyzers: Vec<Arc<dyn Analyzer>>,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            analyzers: Vec::new(),
        }
    }

    pub fn register_analyzer(&mut self, analyzer: Arc<dyn Analyzer>) {
        self.analyzers.push(analyzer);
    }

    pub fn run_on_path(&self, path: impl Into<std::path::PathBuf>) -> Result<AnalysisReport> {
        let contract = Contract::load(path)?;
        self.run_on_contract(&contract)
    }

    pub fn run_on_contract(&self, contract: &Contract) -> Result<AnalysisReport> {
        let mut merged = AnalysisReport::default();
        for a in &self.analyzers {
            let r = a.analyze(contract)?;
            merged.merge(r);
        }
        Ok(merged)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::analyzer::PlaceholderAnalyzer;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;
    use std::sync::Arc;

    #[test]
    fn engine_runs_with_placeholder_analyzer() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("contract.wasm");
        let mut f = File::create(&file_path).unwrap();
        f.write_all(&[0u8, 1, 2, 3]).unwrap();
        f.flush().unwrap();

        let mut engine = Engine::new();
        engine.register_analyzer(Arc::new(PlaceholderAnalyzer::new()));
        let report = engine.run_on_path(file_path).expect("engine run failed");
        assert_eq!(report.issues.len(), 0);
    }
}