#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RunMode {
    Smoke,
    Production,
}

impl RunMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Smoke => "smoke",
            Self::Production => "production",
        }
    }

    pub fn parse(value: &str) -> Result<Self, String> {
        match value {
            "smoke" => Ok(Self::Smoke),
            "production" => Ok(Self::Production),
            other => Err(format!("--mode must be smoke or production, got {other}")),
        }
    }
}

pub fn selected_run_mode(args: &[String]) -> Result<RunMode, String> {
    let mut mode = RunMode::Smoke;
    let mut seen_mode = false;
    let mut index = 0;
    while index < args.len() {
        let (flag, inline_value) = split_inline_arg(args[index].clone());
        if flag == "--mode" {
            if seen_mode {
                return Err("--mode may be provided at most once".to_owned());
            }
            seen_mode = true;
            let (value, consumed_next) = match inline_value {
                Some(value) => (value, false),
                None => (
                    args.get(index + 1)
                        .cloned()
                        .ok_or_else(|| "--mode requires a value".to_owned())?,
                    true,
                ),
            };
            mode = RunMode::parse(&value)?;
            if consumed_next {
                index += 1;
            }
        }
        index += 1;
    }
    Ok(mode)
}

pub fn split_inline_arg(arg: String) -> (String, Option<String>) {
    match arg.split_once('=') {
        Some((flag, value)) => (flag.to_owned(), Some(value.to_owned())),
        None => (arg, None),
    }
}

pub fn take_value(
    flag: &str,
    inline_value: Option<String>,
    args: &mut impl Iterator<Item = String>,
) -> Result<String, String> {
    match inline_value {
        Some(value) => Ok(value),
        None => args
            .next()
            .ok_or_else(|| format!("{flag} requires a value")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn strings(values: &[&str]) -> Vec<String> {
        values.iter().map(|value| value.to_string()).collect()
    }

    #[test]
    fn selected_run_mode_defaults_to_smoke() {
        assert_eq!(selected_run_mode(&[]).unwrap(), RunMode::Smoke);
    }

    #[test]
    fn selected_run_mode_accepts_inline_and_separate_values() {
        assert_eq!(
            selected_run_mode(&strings(&["--mode=production"])).unwrap(),
            RunMode::Production
        );
        assert_eq!(
            selected_run_mode(&strings(&["--mode", "smoke"])).unwrap(),
            RunMode::Smoke
        );
    }

    #[test]
    fn selected_run_mode_rejects_missing_unknown_and_duplicate_values() {
        assert!(selected_run_mode(&strings(&["--mode"])).is_err());
        assert!(selected_run_mode(&strings(&["--mode", "trial"])).is_err());
        assert!(selected_run_mode(&strings(&["--mode", "smoke", "--mode", "production"])).is_err());
    }
}
