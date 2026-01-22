# GasGuard - Usage Guide

## Overview

GasGuard is a static analysis tool specifically designed for Stellar Soroban contracts that identifies unused state variables to help developers minimize storage footprint and reduce ledger rent costs.

## Installation

```bash
# Build the project
cargo build --release

# The binary will be available at target/release/gasguard.exe on Windows
```

## Usage

### Scan a Single Contract File

```bash
# Basic scan with console output
cargo run -- scan examples/sample_contract.rs

# JSON output for CI/CD integration
cargo run -- scan examples/sample_contract.rs --format json
```

### Scan a Directory of Contracts

```bash
# Scan all Rust files in a directory
cargo run -- scan-dir examples/

# JSON output for batch processing
cargo run -- scan-dir examples/ --format json
```

### Analyze Storage Optimization Potential

```bash
# Get detailed analysis of storage savings
cargo run -- analyze examples/sample_contract.rs

# Analyze entire project
cargo run -- analyze examples/
```

## Example Output

### Console Output

```
🔍 Scanning file: "examples/sample_contract.rs"

⚠️  3 Warnings:
  [WARNING]
  📍 Line 8: unused_counter
  📝 State variable 'unused_counter' is declared but never used in contract 'TokenContract'. This wastes storage space and increases ledger rent costs.
  💡 Consider removing the unused state variable 'unused_counter' or implement functionality that uses it. If it's reserved for future use, add a comment explaining its purpose.

  [WARNING]
  📍 Line 8: deprecated_feature
  📝 State variable 'deprecated_feature' is declared but never used in contract 'TokenContract'. This wastes storage space and increases ledger rent costs.
  💡 Consider removing the unused state variable 'deprecated_feature' or implement functionality that uses it. If it's reserved for future use, add a comment explaining its purpose.

  [WARNING]
  📍 Line 8: future_upgrade_slot
  📝 State variable 'future_upgrade_slot' is declared but never used in contract 'TokenContract'. This wastes storage space and increases ledger rent costs.
  💡 Consider removing the unused state variable 'future_upgrade_slot' or implement functionality that uses it. If it's reserved for future use, add a comment explaining its purpose.

Scan Summary: 3 total violations (0 errors, 3 warnings, 0 info)

💰 Storage Optimization Potential:
   • 3 unused state variables
   • 7.5 KB storage savings
   • 0.0075 XLM/month ledger rent savings
```

### JSON Output

```json
{
  "source": "examples/sample_contract.rs",
  "violations": [
    {
      "rule_name": "unused-state-variables",
      "description": "State variable 'unused_counter' is declared but never used in contract 'TokenContract'. This wastes storage space and increases ledger rent costs.",
      "severity": "Warning",
      "line_number": 8,
      "column_number": 4,
      "variable_name": "unused_counter",
      "suggestion": "Consider removing the unused state variable 'unused_counter' or implement functionality that uses it. If it's reserved for future use, add a comment explaining its purpose."
    }
  ],
  "scan_time": "2024-01-22T10:00:00Z"
}
```

## Integration with CI/CD

### GitHub Actions Example

```yaml
name: GasGuard Analysis
on: [push, pull_request]

jobs:
  gasguard:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Build GasGuard
        run: cargo build --release
      - name: Run GasGuard Analysis
        run: |
          ./target/release/gasguard scan-dir contracts/ --format json > gasguard-report.json
      - name: Upload Report
        uses: actions/upload-artifact@v2
        with:
          name: gasguard-report
          path: gasguard-report.json
```

## Rule Details

### Unused State Variables Rule

- **Rule Name**: `unused-state-variables`
- **Severity**: Warning
- **Description**: Identifies state variables in Soroban contracts that are declared but never read or written to
- **Impact**: Reduces storage footprint and ledger rent costs
- **Detection Method**: AST analysis of contract struct definitions and their implementations

## Cost Savings

On Stellar network, storage costs contribute significantly to ledger rent:

- Each unused variable wastes storage space
- Storage rent is charged continuously for the lifetime of the contract
- Removing unused variables can save 15-30% on transaction costs

## Contributing

To add new optimization rules:

1. Create a new rule struct implementing the `Rule` trait
2. Add the rule to `RuleEngine` in `scanner.rs`
3. Write tests in the rule's module
4. Update documentation

## License

MIT License - see LICENSE file for details.
