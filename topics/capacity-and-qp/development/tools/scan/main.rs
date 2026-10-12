mod args;
mod input;
mod output;

fn main() {
    let args = args::parse_args();
    let options = input::LoadCaseOptions {
        input_source: args.input_source,
        max_rows_per_family: args.max_rows_per_family,
        generated_samples_per_facet: args.generated_samples_per_facet,
        generated_seed: args.generated_seed,
        family_filter: args.family_filter,
        source_id_filter: args.source_id_filter,
    };
    let cases = input::load_cases(&options);
    let total = output::write_scan_rows(
        &args.output,
        cases,
        args.audit_generated,
        args.audit_preprocessed,
        args.max_audit_rows,
        args.validation_policy,
        args.capacity_method,
        args.near_redundant_facet_removal,
        args.near_redundant_facet_removal_delta,
    );
    eprintln!("wrote {total} rows to {}", args.output.display());
}
