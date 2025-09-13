use crate::prelude::*;

pub fn glob_base<S: AsRef<str>>(pattern: S) -> Result<PathBuf, Error> {
    let path = Path::new(pattern.as_ref());

    let mut parts = Vec::new();
    for comp in path.components() {
        let s = comp.as_os_str().to_string_lossy().to_string();
        if s.contains('*') || s.contains('?') || s.contains('[') {
            break;
        }
        parts.push(s);
    }

    if parts.is_empty() {
        return Err(anyhow!(
            "Could not determine base dir from the glob pattern: {}",
            pattern.as_ref()
        ));
    }

    Ok(parts.into_iter().collect())
}
