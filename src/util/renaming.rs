use crate::prelude::*;

pub fn handle_duplicate_renaming(dest: &Path, format: &str) -> Result<PathBuf, Error> {
    let mut counter: usize = 1;
    let stem = dest.file_stem().unwrap_or_default().to_string_lossy();
    let ext = dest.extension().unwrap_or_default().to_string_lossy();

    while PathBuf::from(
        format
            .replace("{stem}", &stem)
            .replace("{ext}", &ext)
            .replace("{n}", &counter.to_string()),
    )
    .exists()
    {
        counter = counter.checked_add(1).ok_or(anyhow!(
            "Why do you have {} files with the same name?",
            usize::MAX
        ))?;
    }

    Ok(PathBuf::from(
        format
            .replace("{stem}", &stem)
            .replace("{ext}", &ext)
            .replace("{n}", &counter.to_string()),
    ))
}
