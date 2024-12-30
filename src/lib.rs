/*
    Created by Zoltan Kovari, 2024.

    Licensed under the Apache License, Version 2.0
    http://www.apache.org/licenses/LICENSE-2.0
    (see LICENSE.txt)
*/

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

pub fn compare_pdfs(config: Config) -> Result<(), Box<dyn std::error::Error>> {
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
    let right_list: Vec<_> = std::fs::read_dir(&config.right_dir)?.collect();

//~     if left_list.len() != right_list.len() {
//~         eprintln!("Left and right output file count does not match.");
//~         let mut res = String::new();
//~         loop {
//~             eprint!("Do you want to continue? [Y/n] ");
//~             std::io::stderr().flush()?;

//~             res.clear();
//~             std::io::stdin().read_line(&mut res)?;
//~             match res.trim() {
//~                 "" | "y" | "Y" | "yes" | "Yes" | "YES" => {
//~                     break;
//~                 }
//~                 "n" | "N" | "no" | "No" | "NO" => {
//~                     eprintln!("Aborting.");
//~                     return Ok(());
//~                 }
//~                 _ => (),
//~             }
//~         }
//~     }
    eprintln!("left={}, right={}", left_list.len(), right_list.len());

    Ok(())
}
