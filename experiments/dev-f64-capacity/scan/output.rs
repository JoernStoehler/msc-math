use crate::args::{AuditGenerated, AuditSimplified};
use exp_dev_f64_capacity::{
    scan_case_with_options, F64CapacityMethod, F64ValidationPolicy, ProductSimplificationPolicy,
    ScanCase, ScanOptions,
};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

pub(crate) fn write_scan_rows(
    output: &Path,
    cases: impl IntoIterator<Item = ScanCase>,
    audit_generated: AuditGenerated,
    audit_simplified: AuditSimplified,
    max_audit_rows: usize,
    validation_policy: F64ValidationPolicy,
    capacity_method: F64CapacityMethod,
    product_simplification: ProductSimplificationPolicy,
    product_simplification_delta: f64,
) -> usize {
    if let Some(parent) = output.parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent).expect("create output directory");
        }
    }

    let file = File::create(output).expect("create output JSONL");
    let mut writer = BufWriter::new(file);
    let mut total = 0usize;
    let mut exact_audit_rows = 0usize;
    for case in cases {
        let audit_this_case = audit_generated == AuditGenerated::All
            && case.input_source == "generated_f64"
            && (max_audit_rows == 0 || exact_audit_rows < max_audit_rows);
        if audit_this_case {
            exact_audit_rows += 1;
        }
        let audit_simplified_remaining = audit_simplified == AuditSimplified::All
            && (max_audit_rows == 0 || exact_audit_rows < max_audit_rows);
        let row = scan_case_with_options(
            case,
            &ScanOptions {
                audit_generated: audit_this_case,
                audit_simplified: audit_simplified_remaining,
                validation_policy,
                capacity_method,
                product_simplification,
                product_simplification_delta,
            },
        );
        if row.exact_audit_status != "not_requested" && !audit_this_case {
            exact_audit_rows += 1;
        }
        let line = serde_json::to_string(&row).expect("serialize scan row");
        writeln!(writer, "{line}").expect("write scan row");
        total += 1;
    }
    writer.flush().expect("flush output JSONL");
    total
}
