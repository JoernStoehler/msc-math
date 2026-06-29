use std::path::{Path, PathBuf};

pub(crate) struct SysCachePaths {
    pub(crate) inputs: Vec<PathBuf>,
    pub(crate) output: PathBuf,
}

pub(crate) fn sys_cache_paths(
    out_dir: &Path,
    configured_inputs: &[String],
    configured_output: Option<&String>,
) -> SysCachePaths {
    let output = configured_output
        .map(PathBuf::from)
        .unwrap_or_else(|| out_dir.join("sys-computation-cache.jsonl"));
    let mut inputs = configured_inputs
        .iter()
        .map(PathBuf::from)
        .collect::<Vec<_>>();
    if !inputs.iter().any(|path| path == &output) {
        inputs.push(output.clone());
    }
    SysCachePaths { inputs, output }
}
