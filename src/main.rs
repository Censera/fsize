use clap::Parser;
use std::fs;
use std::path::PathBuf;
use std::process;

use fsize::{compute_total_size, format_mtime, format_size, Unit};

#[derive(Parser)]
#[command(
    name = "fsize (filesize)",
    version,
    about = "Display file/directory sizes",
    arg_required_else_help = true
)]
struct Args {
    #[arg(required = true, value_name = "PATH")]
    paths: Vec<PathBuf>,

    #[arg(short = 'b', long = "binary")]
    binary: bool,

    #[arg(short = 'r', long = "raw")]
    raw: bool,

    #[arg(short = 'o', long = "byte")]
    byte: bool,

    #[arg(short = 'i', long = "info")]
    info: bool,

    #[arg(short = 'u', long = "unit", value_name = "UNIT")]
    in_unit: Option<String>,
}

fn main() {
    let args = Args::parse();

    let unit = args.in_unit.as_deref().and_then(Unit::from_str);
    if args.in_unit.is_some() && unit.is_none() {
        eprintln!("[Error] invalid unit `{}`", args.in_unit.unwrap());
        process::exit(1);
    }

    let raw = args.raw || args.byte;
    let mut exit_code = 0;

    for path in &args.paths {
        match compute_total_size(path) {
            Ok(size) => {
                let size_str = if raw {
                    size.to_string()
                } else {
                    format_size(size, unit, args.binary)
                };

                let mut output = size_str;

                if args.info {
                    match fs::symlink_metadata(path) {
                        Ok(meta) => {
                            let ft = meta.file_type();
                            let type_char = if ft.is_dir() {
                                'd'
                            } else if ft.is_symlink() {
                                'l'
                            } else {
                                'f' // might be a file
                            };
                            let mut extra = String::new();
                            extra.push(type_char);
                            extra.push(' ');
                            if let Ok(mtime) = meta.modified() {
                                extra.push_str(&format_mtime(mtime));
                            }
                            output.push(' ');
                            output.push_str(&extra);
                        }
                        Err(e) => {
                            eprintln!(
                                "[Warning] cannot read metadata for `{}`: {}",
                                path.display(),
                                e
                            );
                            exit_code = 1;
                        }
                    }
                }

                if args.paths.len() > 1 {
                    println!("{}\t{}", output, path.display());
                } else {
                    println!("{}", output);
                }
            }
            Err(e) => {
                eprintln!("[Error] {}", e);
                exit_code = 1;
            }
        }
    }

    process::exit(exit_code);
}
