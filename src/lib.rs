use std::error::Error;
use std::fmt;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadedInput {
    pub path: PathBuf,
    pub source: String,
}

#[derive(Debug)]
pub enum InputError {
    NotFound(PathBuf),
    NotAFile(PathBuf),
    UnsupportedExtension(PathBuf),
    Inspect { path: PathBuf, source: io::Error },
    Read { path: PathBuf, source: io::Error },
}

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound(path) => write!(f, "input file does not exist: {}", path.display()),
            Self::NotAFile(path) => write!(f, "input path is not a file: {}", path.display()),
            Self::UnsupportedExtension(path) => write!(
                f,
                "expected a Python source file with a .py extension: {}",
                path.display()
            ),
            Self::Inspect { path, source } => {
                write!(
                    f,
                    "could not inspect input path {}: {source}",
                    path.display()
                )
            }
            Self::Read { path, source } => {
                write!(f, "could not read input file {}: {source}", path.display())
            }
        }
    }
}

impl Error for InputError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Inspect { source, .. } | Self::Read { source, .. } => Some(source),
            Self::NotFound(_) | Self::NotAFile(_) | Self::UnsupportedExtension(_) => None,
        }
    }
}

/// Validates and reads one UTF-8 Python file. The source is never executed.
pub fn load_python_file(path: impl AsRef<Path>) -> Result<LoadedInput, InputError> {
    let path = path.as_ref().to_path_buf();
    let metadata = fs::metadata(&path).map_err(|source| {
        if source.kind() == io::ErrorKind::NotFound {
            InputError::NotFound(path.clone())
        } else {
            InputError::Inspect {
                path: path.clone(),
                source,
            }
        }
    })?;

    if !metadata.is_file() {
        return Err(InputError::NotAFile(path));
    }

    let is_python = path
        .extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| extension.eq_ignore_ascii_case("py"));
    if !is_python {
        return Err(InputError::UnsupportedExtension(path));
    }

    let source = fs::read_to_string(&path).map_err(|source| InputError::Read {
        path: path.clone(),
        source,
    })?;

    Ok(LoadedInput { path, source })
}

#[cfg(test)]
mod tests {
    use super::{InputError, load_python_file};
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_path(label: &str) -> std::path::PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!("py2rust-{label}-{}-{nonce}", std::process::id()))
    }

    #[test]
    fn loads_python_file() {
        let dir = temp_path("valid");
        fs::create_dir_all(&dir).unwrap();
        let path = dir.join("example.py");
        fs::write(&path, "print('hello')\n").unwrap();
        let loaded = load_python_file(&path).unwrap();
        assert_eq!(loaded.source, "print('hello')\n");
        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn reports_missing_file() {
        assert!(matches!(
            load_python_file(temp_path("missing").with_extension("py")),
            Err(InputError::NotFound(_))
        ));
    }

    #[test]
    fn rejects_directory() {
        let path = temp_path("directory.py");
        fs::create_dir_all(&path).unwrap();
        assert!(matches!(
            load_python_file(&path),
            Err(InputError::NotAFile(_))
        ));
        fs::remove_dir_all(path).unwrap();
    }

    #[test]
    fn rejects_wrong_extension() {
        let dir = temp_path("extension");
        fs::create_dir_all(&dir).unwrap();
        let path = dir.join("input.txt");
        fs::write(&path, "text").unwrap();
        assert!(matches!(
            load_python_file(&path),
            Err(InputError::UnsupportedExtension(_))
        ));
        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn reports_invalid_utf8() {
        let dir = temp_path("utf8");
        fs::create_dir_all(&dir).unwrap();
        let path = dir.join("invalid.py");
        fs::write(&path, [0xFF, 0xFE]).unwrap();
        assert!(matches!(
            load_python_file(&path),
            Err(InputError::Read { .. })
        ));
        fs::remove_dir_all(dir).unwrap();
    }
}
