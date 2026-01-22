use gasguard_rules::{RuleEngine, UnusedStateVariablesRule};
use std::path::Path;
use anyhow::{Result, Context};

pub struct ContractScanner {
    rule_engine: RuleEngine,
}

impl ContractScanner {
    pub fn new() -> Self {
        let rule_engine = RuleEngine::new()
            .add_rule(Box::new(UnusedStateVariablesRule));
            
        Self { rule_engine }
    }
    
    pub fn scan_file(&self, file_path: &Path) -> Result<ScanResult> {
        let content = std::fs::read_to_string(file_path)
            .with_context(|| format!("Failed to read file: {:?}", file_path))?;
            
        self.scan_content(&content, file_path.to_string_lossy().to_string())
    }
    
    pub fn scan_content(&self, content: &str, source: String) -> Result<ScanResult> {
        let violations = self.rule_engine.analyze(content)
            .with_context(|| "Failed to analyze contract code")?;
            
        Ok(ScanResult {
            source,
            violations,
            scan_time: chrono::Utc::now(),
        })
    }
    
    pub fn scan_directory(&self, dir_path: &Path) -> Result<Vec<ScanResult>> {
        let mut results = Vec::new();
        
        for entry in walkdir::WalkDir::new(dir_path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| {
                e.path().extension().map_or(false, |ext| ext == "rs")
            }) {
            
            let result = self.scan_file(entry.path())?;
            if !result.violations.is_empty() {
                results.push(result);
            }
        }
        
        Ok(results)
    }
}

#[derive(Debug, Clone)]
pub struct ScanResult {
    pub source: String,
    pub violations: Vec<gasguard_rules::RuleViolation>,
    pub scan_time: chrono::DateTime<chrono::Utc>,
}

impl ScanResult {
    pub fn has_violations(&self) -> bool {
        !self.violations.is_empty()
    }
    
    pub fn get_violations_by_severity(&self, severity: gasguard_rules::ViolationSeverity) -> Vec<&gasguard_rules::RuleViolation> {
        self.violations
            .iter()
            .filter(|v| matches!(v.severity, severity))
            .collect()
    }
    
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}
