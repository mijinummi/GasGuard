use gasguard_engine::{ContractScanner, ScanAnalyzer};
use std::path::Path;

#[test]
fn test_unused_state_variables_detection() {
    let scanner = ContractScanner::new();
    let contract_code = r#"
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env};

#[contracttype]
pub struct TestContract {
    pub used_var: u64,
    pub unused_var: String,
    pub another_used: bool,
}

#[contractimpl]
impl TestContract {
    pub fn new() -> Self {
        Self {
            used_var: 42,
            another_used: true,
            unused_var: "never_used".to_string(),
        }
    }
    
    pub fn get_used_var(&self) -> u64 {
        self.used_var
    }
    
    pub fn set_another_used(&mut self, value: bool) {
        self.another_used = value;
    }
}
"#;

    let result = scanner.scan_content(contract_code, "test_contract.rs".to_string()).unwrap();
    
    // Should detect one unused variable
    assert_eq!(result.violations.len(), 1);
    assert_eq!(result.violations[0].variable_name, "unused_var");
    assert_eq!(result.violations[0].rule_name, "unused-state-variables");
}

#[test]
fn test_optimized_contract_no_violations() {
    let scanner = ContractScanner::new();
    let contract_code = r#"
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env};

#[contracttype]
pub struct OptimizedContract {
    pub counter: u64,
    pub owner: Address,
}

#[contractimpl]
impl OptimizedContract {
    pub fn new(owner: Address) -> Self {
        Self {
            counter: 0,
            owner,
        }
    }
    
    pub fn increment(&mut self) {
        self.counter += 1;
    }
    
    pub fn get_owner(&self) -> &Address {
        &self.owner
    }
}
"#;

    let result = scanner.scan_content(contract_code, "optimized_contract.rs".to_string()).unwrap();
    
    // Should detect no violations
    assert_eq!(result.violations.len(), 0);
}

#[test]
fn test_multiple_unused_variables() {
    let scanner = ContractScanner::new();
    let contract_code = r#"
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env};

#[contracttype]
pub struct WastefulContract {
    pub used_var: u64,
    pub unused1: String,
    pub unused2: bool,
    pub unused3: Address,
    pub also_used: u32,
}

#[contractimpl]
impl WastefulContract {
    pub fn new() -> Self {
        Self {
            used_var: 42,
            also_used: 100,
            unused1: "unused".to_string(),
            unused2: false,
            unused3: Address::generate(&Env::default()),
        }
    }
    
    pub fn get_used_var(&self) -> u64 {
        self.used_var
    }
    
    pub fn get_also_used(&self) -> u32 {
        self.also_used
    }
}
"#;

    let result = scanner.scan_content(contract_code, "wasteful_contract.rs".to_string()).unwrap();
    
    // Should detect three unused variables
    assert_eq!(result.violations.len(), 3);
    
    let unused_vars: Vec<String> = result.violations.iter()
        .map(|v| v.variable_name.clone())
        .collect();
    
    assert!(unused_vars.contains(&"unused1".to_string()));
    assert!(unused_vars.contains(&"unused2".to_string()));
    assert!(unused_vars.contains(&"unused3".to_string()));
}

#[test]
fn test_storage_savings_calculation() {
    let violations = vec![
        gasguard_rules::RuleViolation {
            rule_name: "unused-state-variables".to_string(),
            description: "Test violation 1".to_string(),
            severity: gasguard_rules::ViolationSeverity::Warning,
            line_number: 10,
            column_number: 4,
            variable_name: "unused_var1".to_string(),
            suggestion: "Remove it".to_string(),
        },
        gasguard_rules::RuleViolation {
            rule_name: "unused-state-variables".to_string(),
            description: "Test violation 2".to_string(),
            severity: gasguard_rules::ViolationSeverity::Warning,
            line_number: 11,
            column_number: 4,
            variable_name: "unused_var2".to_string(),
            suggestion: "Remove it".to_string(),
        },
    ];
    
    let savings = ScanAnalyzer::calculate_storage_savings(&violations);
    
    assert_eq!(savings.unused_variables, 2);
    assert_eq!(savings.estimated_savings_kb, 5.0); // 2 * 2.5 KB per variable
    assert!(savings.monthly_ledger_rent_savings > 0.0);
}
