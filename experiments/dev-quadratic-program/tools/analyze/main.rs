mod args;
mod input;
mod summary;

fn main() {
    let args = args::parse_args();
    let rows = input::read_rows(&args.input);
    let summary = summary::summarize(rows);
    summary::print_summary(&summary);
    if let Some(path) = args.json_output {
        summary::write_json_summary(&path, &summary);
    }
}
