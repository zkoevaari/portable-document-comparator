/*
    Created by Zoltan Kovari, 2024.

    Licensed under the Apache License, Version 2.0
    http://www.apache.org/licenses/LICENSE-2.0
    (see LICENSE.txt)
*/

use portable_document_comparator::*;

use std::error::Error;
use std::io::Write;
use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
struct Args {
    ///Left-side input PDF file
    left_file: PathBuf,

    ///Right-side input PDF file
    right_file: PathBuf,

    ///Output directory; if omitted, current directory is used
    #[arg(default_value = ".")]
    out_dir: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    match args.left_file.try_exists() {
        Err(e) => return Err(format!("could not check left file: {}", e.kind()).into()),
        Ok(false) => return Err("left file does not exist".into()),
        Ok(true) if !args.left_file.is_file() => {
            return Err("the left side is not a regular file".into());
        }
        _ => (),
    }
    match args.right_file.try_exists() {
        Err(e) => return Err(format!("could not check right file: {}", e.kind()).into()),
        Ok(false) => return Err("right file does not exist".into()),
        Ok(true) if !args.right_file.is_file() => {
            return Err("the right side is not a regular file".into());
        }
        _ => (),
    }
    match args.out_dir.try_exists() {
        Err(e) => return Err(format!("could not check output directory: {}", e.kind()).into()),
        Ok(true) if !args.out_dir.is_dir() => return Err("output is not a directory".into()),
        Ok(_) => (),
    }

    eprint!("Creating output directories if necessary...");
    std::io::stderr().flush()?;

    let left_dir = args.out_dir.join("left");
    match std::fs::create_dir_all(&left_dir) {
        Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => (),
        Err(e) => return Err(e.into()),
        _ => (),
    }

    let right_dir = args.out_dir.join("right");
    match std::fs::create_dir(&right_dir) {
        Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => (),
        Err(e) => return Err(e.into()),
        _ => (),
    }

    let diff_dir = args.out_dir.join("diff");
    match std::fs::create_dir(&diff_dir) {
        Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => (),
        Err(e) => return Err(e.into()),
        _ => (),
    }

    eprintln!("Ok.");

    match check_dirs_empty(&left_dir, &right_dir, &diff_dir) {
        Err(e) => return Err(format!("could not check output directories: {}", e.kind()).into()),
        Ok(true) => (),
        Ok(false) => {
            eprintln!(
                "Some output directories are not empty. \
                We will clear those if we continue."
            );
            let mut res = String::new();
            loop {
                eprint!("Do you agree? [Y/n] ");
                std::io::stderr().flush()?;

                res.clear();
                std::io::stdin().read_line(&mut res)?;
                match res.trim() {
                    "" | "y" | "Y" | "yes" | "Yes" | "YES" => {
                        break;
                    }
                    "n" | "N" | "no" | "No" | "NO" => {
                        eprintln!("Aborting.");
                        return Ok(());
                    }
                    _ => (),
                }
            }

            eprint!("Clearing output directories...");
            std::io::stderr().flush()?;

            clear_directory(&left_dir)?;
            clear_directory(&right_dir)?;
            clear_directory(&diff_dir)?;

            eprintln!("Ok.");
        }
    }

    compare_pdfs(Config {
        left_file: args.left_file,
        right_file: args.right_file,
        left_dir,
        right_dir,
        diff_dir,
    })
}

fn check_dirs_empty(
    left_dir: &PathBuf,
    right_dir: &PathBuf,
    diff_dir: &PathBuf,
) -> Result<bool, std::io::Error> {
    let left = std::fs::read_dir(left_dir)?.next().transpose()?;
    let right = std::fs::read_dir(right_dir)?.next().transpose()?;
    let diff = std::fs::read_dir(diff_dir)?.next().transpose()?;

    match (left, right, diff) {
        (Some(_), _, _) | (_, Some(_), _) | (_, _, Some(_)) => Ok(false),
        _ => Ok(true),
    }
}

fn clear_directory(dir: &PathBuf) -> Result<(), Box<dyn Error>> {
    let list: Vec<_> = std::fs::read_dir(dir)?.collect();
    for r in list {
        let entry: std::fs::DirEntry = r?;
        if entry.file_type()?.is_file() {
            std::fs::remove_file(entry.path())?;
        } else {
            return Err("output directory contains non-regular file(s)".into());
        }
    }
    Ok(())
}
