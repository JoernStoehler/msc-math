use std::process::ExitCode;

fn main() -> ExitCode {
    match exp_dev_qp_numerics_audit::run_from_env() {
        Ok(out_dir) => {
            println!("{}", out_dir.display());
            ExitCode::SUCCESS
        }
        Err(message) => {
            eprintln!("{message}");
            ExitCode::FAILURE
        }
    }
}
