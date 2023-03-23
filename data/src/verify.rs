use std::path::Path;

pub fn is_mac_application(path: &Path) -> bool {
    path.extension().map(|ext| ext == "app").unwrap_or(false)
}
