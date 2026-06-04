//! Plain path helpers for the regular-products experiment package.

use std::path::PathBuf;

pub fn package_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

pub fn experiment_path(experiment_dir: &str, file_name: &str) -> PathBuf {
    package_root().join(experiment_dir).join(file_name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn experiment_path_points_inside_regular_products_package() {
        let path = experiment_path("pentagon-rotation-empirics", "theta-sweep.jsonl");
        let rendered = path.to_string_lossy();
        assert!(rendered.contains("experiments/regular-products"));
        assert!(rendered.ends_with("pentagon-rotation-empirics/theta-sweep.jsonl"));
    }
}
