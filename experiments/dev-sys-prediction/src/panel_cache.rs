use std::path::{Path, PathBuf};

pub(crate) struct SysextCachePaths {
    pub(crate) inputs: Vec<PathBuf>,
    pub(crate) output: PathBuf,
}

pub(crate) fn sysext_cache_paths(
    out_dir: &Path,
    configured_inputs: &[String],
    configured_output: Option<&String>,
) -> SysextCachePaths {
    let output = configured_output
        .map(PathBuf::from)
        .unwrap_or_else(|| out_dir.join("sysext-cache.jsonl"));
    let inputs = configured_inputs
        .iter()
        .map(PathBuf::from)
        .collect::<Vec<_>>();
    SysextCachePaths { inputs, output }
}
