mod diff;
mod patch;

use crate::patch::Patch;
use std::path::PathBuf;

const DEFAULT_OUTPUT_PATCH_NAME: &str = "out.patch";
const DEFAULT_OUTPUT_FILE_NAME: &str = "file.applied";

pub fn create_patch(
    old_file: PathBuf,
    new_file: PathBuf,
    output: Option<PathBuf>,
) -> std::io::Result<()> {
    let output = output.unwrap_or(DEFAULT_OUTPUT_PATCH_NAME.into());

    let old_bytes = std::fs::read(&old_file)?;
    let new_bytes = std::fs::read(&new_file)?;

    let patch = crate::patch::create(old_bytes, new_bytes)?;

    let serialized_patch = patch.into_bytes();

    std::fs::write(output, serialized_patch)
}

pub fn apply_patch(file: PathBuf, patch: PathBuf, output: Option<PathBuf>) -> std::io::Result<()> {
    let output = output.unwrap_or(DEFAULT_OUTPUT_FILE_NAME.into());

    let bytes = std::fs::read(file)?;
    let serialized_patch = std::fs::read(&patch)?;

    let patch = Patch::from_bytes(&serialized_patch)?;

    let result = crate::patch::apply(bytes, patch)?;

    std::fs::write(output, result)
}
