use std::path::{Path, PathBuf};
use std::{fs, io};

#[derive(Debug, thiserror::Error)]

pub enum FsizeError {
    #[error("I/O error for `{path}`: {source}")]
    Io { path: PathBuf, source: io::Error },

    #[error("Invalid unit specification: `{0}`")]
    InvalidUnit(String),

    #[error("Path does not exist: `{0}`")]
    NotFound(PathBuf),
}

pub fn compute_total_size(path: &Path) -> Result<u64, FsizeError> {
    let meta = fs::symlink_metadata(path).map_err(|e| FsizeError::Io {
        path: path.to_owned(),
        source: e,
    })?;

    if meta.file_type().is_dir() {
        let mut total = 0;
        let mut stack = Vec::new();
        stack.push(path.to_owned());

        while let Some(dir) = stack.pop() {
            match fs::read_dir(&dir) {
                Ok(entries) => {
                    for entry in entries {
                        match entry {
                            Ok(e) => {
                                let p = e.path();
                                match fs::symlink_metadata(&p) {
                                    Ok(m) => {
                                        if m.file_type().is_dir() {
                                            stack.push(p);
                                        } else {
                                            total += m.len();
                                        }
                                    }
                                    Err(e) => {
                                        eprintln!(
                                            "Warning: cannot access `{}`: {}",
                                            p.display(),
                                            e
                                        );
                                    }
                                }
                            }
                            Err(e) => {
                                eprintln!("Warning: read_dir entry error: {}", e);
                            }
                        }
                    }
                }
                Err(e) => {
                    return Err(FsizeError::Io {
                        path: dir,
                        source: e,
                    });
                }
            }
        }
        Ok(total)
    } else {
        Ok(meta.len())
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Unit {
    B,
    KB,
    MB,
    GB,
    TB,
    KiB,
    MiB,
    GiB,
    TiB,
    //PB, PiB, etc. // Maybe in the future
}

impl Unit {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.trim().to_uppercase().as_str() {
            "B" => Some(Unit::B),
            "KB" => Some(Unit::KB),
            "MB" => Some(Unit::MB),
            "GB" => Some(Unit::GB),
            "TB" => Some(Unit::TB),
            "KIB" => Some(Unit::KiB),
            "MIB" => Some(Unit::MiB),
            "GIB" => Some(Unit::GiB),
            "TIB" => Some(Unit::TiB),
            _ => None,
        }
    }

    const fn divisor(self) -> u64 {
        match self {
            Unit::B => 1,
            Unit::KB => 1000,
            Unit::MB => 1000_000,
            Unit::GB => 1000_000_000,
            Unit::TB => 1000_000_000_000,
            Unit::KiB => 1024,
            Unit::MiB => 1024 * 1024,
            Unit::GiB => 1024 * 1024 * 1024,
            Unit::TiB => 1024 * 1024 * 1024 * 1024,
        }
    }

    fn name(self) -> &'static str {
        match self {
            Unit::B => "B",
            Unit::KB => "KB",
            Unit::MB => "MB",
            Unit::GB => "GB",
            Unit::TB => "TB",
            Unit::KiB => "KiB",
            Unit::MiB => "MiB",
            Unit::GiB => "GiB",
            Unit::TiB => "TiB",
        }
    }
}

pub fn format_size(bytes: u64, unit: Option<Unit>, binary: bool) -> String {
    let unit = unit.unwrap_or_else(|| {
        if binary {
            if bytes >= Unit::TiB.divisor() {
                Unit::TiB
            } else if bytes >= Unit::GiB.divisor() {
                Unit::GiB
            } else if bytes >= Unit::MiB.divisor() {
                Unit::MiB
            } else if bytes >= Unit::KiB.divisor() {
                Unit::KiB
            } else {
                Unit::B
            }
        } else {
            if bytes >= Unit::TB.divisor() {
                Unit::TB
            } else if bytes >= Unit::GB.divisor() {
                Unit::GB
            } else if bytes >= Unit::MB.divisor() {
                Unit::MB
            } else if bytes >= Unit::KB.divisor() {
                Unit::KB
            } else {
                Unit::B
            }
        }
    });

    let divisor = unit.divisor();
    if divisor == 1 || bytes < divisor {
        format!("{} {}", bytes, unit.name())
    } else {
        let value = bytes as f64 / divisor as f64;
        if value.fract().abs() < 0.005 {
            format!("{} {}", value.round() as u64, unit.name())
        } else {
            format!("{:.1} {}", value, unit.name())
        }
    }
}

use chrono::{DateTime, Local};

pub fn format_mtime(time: std::time::SystemTime) -> String {
    let dt: DateTime<Local> = time.into();
    dt.format("%b %e %H:%M").to_string()
}
