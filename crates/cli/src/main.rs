use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use crate::{
    commands::{Commands, RenameCommands},
    error::Result,
};
use clap::Parser;
use regex::Regex;
use walkdir::WalkDir;
use zip_extensions::{zip_create_from_directory, zip_extract};

mod commands;
mod error;

fn rename_tzp<I: AsRef<Path>, O: AsRef<Path>>(input: I, output: O) -> Result<()> {
    let temp_dir = tempdir::TempDir::new("cbz-rename")?;
    zip_extract(
        &input.as_ref().to_path_buf(),
        &temp_dir.path().to_path_buf(),
    )?;
    // find all the cbz files that
    let files: Vec<PathBuf> = WalkDir::new(temp_dir.path())
        .into_iter()
        .filter_map(|entry| {
            let entry = entry.ok()?;
            if entry.file_type().is_file() && entry.path().extension().is_some_and(|e| e == "cbz") {
                Some(entry.into_path())
            } else {
                None
            }
        })
        .collect();
    for file in files {
        println!("{:?}", file.file_name().unwrap());
        rename_tzp_volume(&file, &output)?;
    }

    Ok(())
}

fn rename_tzp_volume<O: AsRef<Path>>(input: &PathBuf, output: O) -> Result<()> {
    let temp_dir = tempdir::TempDir::new("volume-rename")?;
    zip_extract(input, &temp_dir.path().to_path_buf())?;
    let files: Vec<PathBuf> = WalkDir::new(temp_dir.path())
        .into_iter()
        .filter_map(|entry| {
            let entry = entry.ok()?;
            if entry.file_type().is_file() {
                Some(entry.into_path())
            } else {
                None
            }
        })
        .collect();
    for (chapter, files) in group_by_chapter(files.iter()) {
        let chapter_name = format!("ch{chapter:04}");
        let chapter_path = temp_dir.path().join(&chapter_name);
        std::fs::create_dir_all(&chapter_path)?;
        for (i, file) in files.iter().enumerate() {
            let ext = file.extension().unwrap().to_str().unwrap();
            std::fs::rename(file, chapter_path.join(format!("{:02}.{}", i + 1, ext)))?;
        }
        let output_path = output.as_ref().join(format!("{chapter_name}.cbz"));
        zip_create_from_directory(&output_path, &chapter_path)?;
    }
    Ok(())
}

fn group_by_chapter<'a, I>(iter: I) -> HashMap<usize, Vec<&'a PathBuf>>
where
    I: Iterator<Item = &'a PathBuf>,
{
    let pattern = Regex::new(r".*_(\d+)_(\d+)\.(\w+)").unwrap();
    let mut results = HashMap::new();

    for item in iter {
        if let Some(captures) = pattern.captures(item.file_name().unwrap().to_str().unwrap()) {
            let group_num = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
            results.entry(group_num).or_insert_with(Vec::new).push(item);
        }
    }

    for files in results.values_mut() {
        files.sort();
    }

    results
}

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let args = commands::Args::parse();
    match args.command {
        Commands::Rename(hello) => {
            let cmd = hello.command;
            match cmd {
                RenameCommands::Tzp => {
                    rename_tzp(hello.input, hello.output_dir)?;
                }
            }
        }
    }

    Ok(())
}
