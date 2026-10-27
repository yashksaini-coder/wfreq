//! Turns the paths given on the command line into a list of files to read.

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

#[cfg(test)]
mod tests {
    use super::*;

    /// Creates a fresh folder tree for one test:
    /// root/a.txt, root/b.md, root/skip.csv, root/sub/c.txt
    fn tree(name: &str) -> PathBuf {
        let root = std::env::temp_dir().join(format!("wfreq-input-{name}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(root.join("sub")).unwrap();
        for f in ["a.txt", "b.md", "skip.csv", "sub/c.txt"] {
            fs::write(root.join(f), "word").unwrap();
        }
        root
    }

    fn names(files: &[PathBuf], root: &Path) -> Vec<String> {
        files
            .iter()
            .map(|f| {
                f.strip_prefix(root)
                    .unwrap()
                    .to_string_lossy()
                    .replace('\\', "/")
            })
            .collect()
    }

    #[test]
    fn folder_without_recursive_reads_top_level_only() {
        let root = tree("flat");
        let files = collect_files(std::slice::from_ref(&root), false).unwrap();
        assert_eq!(names(&files, &root), ["a.txt", "b.md"]);
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn folder_with_recursive_reads_subfolders() {
        let root = tree("deep");
        let files = collect_files(std::slice::from_ref(&root), true).unwrap();
        assert_eq!(names(&files, &root), ["a.txt", "b.md", "sub/c.txt"]);
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn plain_paths_and_stdin_are_kept_as_given() {
        let paths = [PathBuf::from("missing.pdf"), PathBuf::from("-")];
        assert_eq!(collect_files(&paths, false).unwrap(), paths);
    }
}
