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

pub struct Color;

impl Color {
    pub const RED: &'static str = "\x1b[1;31m";
    pub const YELLOW: &'static str = "\x1b[33m";
    pub const RESET: &'static str = "\x1b[0m";
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
                                            "{}[WARNING]{} cannot access `{}`: {}",
                                            Color::YELLOW,
                                            Color::RESET,
                                            p.display(),
                                            e
                                        );
                                    }
                                }
                            }
                            Err(e) => {
                                eprintln!(
                                    "{}[WARNING]{} read_dir entry error: {}",
                                    Color::YELLOW,
                                    Color::RESET,
                                    e
                                );
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
        match s.trim().to_lowercase().as_str() {
            "b" => Some(Unit::B),
            "kb" => Some(Unit::KB),
            "mb" => Some(Unit::MB),
            "gb" => Some(Unit::GB),
            "tb" => Some(Unit::TB),
            "kib" => Some(Unit::KiB),
            "mib" => Some(Unit::MiB),
            "gib" => Some(Unit::GiB),
            "tib" => Some(Unit::TiB),
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
    if divisor == 1 {
        format!("{} {}", bytes, unit.name())
    } else {
        let value = bytes as f64 / divisor as f64;
        if value.fract().abs() < 0.005 {
            format!("{} {}", format_pre(value), unit.name())
        } else {
            format!("{} {}", format_pre(value), unit.name())
        }
    }
}

fn format_pre(num: f64) -> String {
    let formatted = format!("{:.1$}", num, 3); // 3 looks nice

    formatted
        .trim_end_matches('0')
        .trim_end_matches('.')
        .to_string()
}

use chrono::{DateTime, Local};

pub fn format_mtime(time: std::time::SystemTime) -> String {
    let dt: DateTime<Local> = time.into();
    dt.format("%b %e %H:%M").to_string()
}
