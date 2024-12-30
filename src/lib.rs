/*
    Created by Zoltan Kovari, 2024.

    Licensed under the Apache License, Version 2.0
    http://www.apache.org/licenses/LICENSE-2.0
    (see LICENSE.txt)
*/

use std::path::PathBuf;

#[derive(Debug)]
pub struct Config {
    pub left_file: PathBuf,
    pub right_file: PathBuf,
    pub left_dir: PathBuf,
    pub right_dir: PathBuf,
    pub diff_dir: PathBuf,
}

pub fn compare_pdfs(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    println!("{:?}", config);

    Ok(())
}
