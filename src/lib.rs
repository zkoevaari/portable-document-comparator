/*
    Created by Zoltan Kovari, 2024.

    Licensed under the Apache License, Version 2.0
    http://www.apache.org/licenses/LICENSE-2.0
    (see LICENSE.txt)
*/

use std::error::Error;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

#[derive(Debug)]
pub struct Config {
    pub left_file: PathBuf,
    pub right_file: PathBuf,
    pub left_dir: PathBuf,
    pub right_dir: PathBuf,
    pub diff_dir: PathBuf,
}

pub fn compare_pdfs(config: Config) -> Result<(), Box<dyn Error>> {
    eprintln!("Executing left-side conversion...");
    let status = Command::new("convert")
        .args([
            "-density",
            "150",
            "-alpha",
            "remove",
            "-alpha",
            "off",
            &config.left_file.to_string_lossy(),
            &config.left_dir.join("%03d.png").to_string_lossy(),
        ])
        .status()?;
    eprintln!("...Done, {}", status);

    eprintln!("Executing right-side conversion...");
    let status = Command::new("convert")
        .args([
            "-density",
            "150",
            "-alpha",
            "remove",
            "-alpha",
            "off",
            &config.right_file.to_string_lossy(),
            &config.right_dir.join("%03d.png").to_string_lossy(),
        ])
        .status()?;
    eprintln!("...Done, {}", status);

    let left_list: Vec<_> = std::fs::read_dir(&config.left_dir)?.collect();
    if let Some(r) = left_list.iter().find(|r| r.is_err()) {
        return Err(format!(
            "could not access some of the converted files: {}",
            r.as_ref().unwrap_err(),
        )
        .into());
    }
    let left_list: Vec<_> = left_list.into_iter().map(|r| r.unwrap().path()).collect();

    let right_list: Vec<_> = std::fs::read_dir(&config.right_dir)?.collect();
    if let Some(r) = right_list.iter().find(|r| r.is_err()) {
        return Err(format!(
            "could not access some of the converted files: {}",
            r.as_ref().unwrap_err(),
        )
        .into());
    }
    let right_list: Vec<_> = right_list.into_iter().map(|r| r.unwrap().path()).collect();

    if left_list.len() != right_list.len() {
        eprintln!("Left and right output file count does not match.");
        if !confirm_yes_no("Do you want to continue?")? {
            eprintln!("Aborting.");
            return Ok(());
        }
    }

    eprint!("Starting comparison...");
    std::io::stderr().flush()?;

    let mut n = 1;
    for lp in left_list {
        if let Some(rp) = right_list
            .iter()
            .find(|rp| rp.file_name() == lp.file_name())
        {
            let filename = lp.file_name().unwrap();

            let status = Command::new("compare")
                .args([
                    "-fuzz",
                    "1000",
                    &lp.to_string_lossy(),
                    &rp.to_string_lossy(),
                    &config.diff_dir.join(filename).to_string_lossy(),
                ])
                .status()?;
            match status.code() {
                Some(2) | None => {
                    return Err(format!(
                        "failure at entry #{} '{}', {}",
                        n,
                        filename.to_string_lossy(),
                        status
                    )
                    .into());
                }
                _ => (),
            }

            n += 1;
        }
    }

    eprintln!("Done, number of pairs processed: {}", n - 1);

    Ok(())
}

pub fn confirm_yes_no(msg: &str) -> Result<bool, Box<dyn Error>> {
    let mut res = String::new();
    loop {
        eprint!("{} [Y/n] ", msg);
        std::io::stderr().flush()?;

        res.clear();
        std::io::stdin().read_line(&mut res)?;
        match res.trim() {
            "" | "y" | "Y" | "yes" | "Yes" | "YES" => break Ok(true),
            "n" | "N" | "no" | "No" | "NO" => break Ok(false),
            _ => (),
        }
    }
}
