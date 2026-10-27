use crate::error::CliError;
use crate::parser;
use std::path::{Path, PathBuf};
use std::{fs, io};

/// Expands `paths` into the files to read, in a stable order.
///
/// - A file (or `-` for stdin) is kept as given; the parser checks its type later.
/// - A directory adds the supported files directly inside it. With `recursive`,
///   it also adds supported files from all its subdirectories.
/// - Unsupported files found inside directories are skipped silently.
pub fn collect_files(paths: &[PathBuf], recursive: bool) -> Result<Vec<PathBuf>, CliError> {
    let mut files = Vec::new();
    for path in paths {
        if path.is_dir() {
            walk_dir(path, recursive, &mut files)?;
        } else {
            files.push(path.clone());
        }
    }
    Ok(files)
}

fn walk_dir(dir: &Path, recursive: bool, files: &mut Vec<PathBuf>) -> Result<(), CliError> {
    let read_err = |source: io::Error| CliError::FileRead {
        path: dir.to_path_buf(),
        source,
    };

    let mut entries = fs::read_dir(dir)
        .map_err(read_err)?
        .map(|entry| entry.map(|e| e.path()))
        .collect::<Result<Vec<_>, _>>()
        .map_err(read_err)?;
    // read_dir returns entries in no particular order; sort so output is the same every run
    entries.sort();

    for path in entries {
        if path.is_dir() {
            // Skip symlinked directories so a link loop cannot recurse forever
            if recursive && !path.is_symlink() {
                walk_dir(&path, recursive, files)?;
            }
        } else if parser::is_supported(&path) {
            files.push(path);
        }
    }
    Ok(())
}
