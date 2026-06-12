use std::process::ExitCode;

fn main() -> ExitCode {
    match exp_numerics::run_from_env() {
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
