use anyhow::{Context, Result};
use clap::Parser;
use colored::*;
use dialoguer::{theme::ColorfulTheme, MultiSelect};
use regex::Regex;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// CLI tool to combine multiple files into a single text file
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Use regex pattern to select files
    #[clap(short, long)]
    regex: Option<String>,

    /// Use interactive selection
    #[clap(short, long)]
    interactive: bool,

    /// Output file path
    #[clap(short, long, default_value = "combined.txt")]
    output: String,

    /// Limit search to specific directories (comma separated)
    #[clap(short, long)]
    dirs: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    if !args.interactive && args.regex.is_none() {
        println!("{}", "Error: You must specify either --regex or --interactive".red());
        return Ok(());
    }

    // Collect paths to search
    let search_dirs = if let Some(dirs) = args.dirs.as_ref() {
        dirs.split(',')
            .map(|dir| PathBuf::from(dir.trim()))
            .collect::<Vec<_>>()
    } else {
        vec![PathBuf::from(".")]
    };

    // Find all files
    let mut all_files = Vec::new();
    for dir in search_dirs {
        if !dir.exists() {
            println!("{} {}", "Warning: Directory not found:".yellow(), dir.display());
            continue;
        }

        for entry in WalkDir::new(dir).into_iter().filter_map(Result::ok) {
            if entry.file_type().is_file() {
                all_files.push(entry.path().to_path_buf());
            }
        }
    }

    if all_files.is_empty() {
        println!("{}", "No files found in the specified paths.".yellow());
        return Ok(());
    }

    // Filter files by regex if provided
    let filtered_files = if let Some(pattern) = args.regex.as_ref() {
        let regex = Regex::new(pattern).context("Invalid regex pattern")?;
        all_files
            .into_iter()
            .filter(|path| {
                if let Some(path_str) = path.to_str() {
                    regex.is_match(path_str)
                } else {
                    false
                }
            })
            .collect::<Vec<_>>()
    } else {
        all_files
    };

    if filtered_files.is_empty() {
        println!("{}", "No files matched the specified pattern.".yellow());
        return Ok(());
    }

    // Final list of files to combine
    let files_to_combine = if args.interactive {
        // Convert paths to relative for better display
        let current_dir = std::env::current_dir()?;
        let relative_paths: Vec<String> = filtered_files
            .iter()
            .map(|path| {
                path.strip_prefix(&current_dir)
                    .unwrap_or(path)
                    .to_string_lossy()
                    .to_string()
            })
            .collect();

        // Interactive selection
        let selections = MultiSelect::with_theme(&ColorfulTheme::default())
            .with_prompt("Select files to combine")
            .items(&relative_paths)
            .defaults(&vec![true; relative_paths.len()])
            .interact()?;

        selections
            .into_iter()
            .map(|i| filtered_files[i].clone())
            .collect()
    } else {
        filtered_files
    };

    if files_to_combine.is_empty() {
        println!("{}", "No files selected for combining.".yellow());
        return Ok(());
    }

    // Combine selected files
    combine_files(&files_to_combine, &args.output)?;

    println!(
        "{} {} {} {}",
        "Successfully combined".green(),
        files_to_combine.len(),
        "files into".green(),
        args.output
    );

    Ok(())
}

fn combine_files(files: &[PathBuf], output_path: &str) -> Result<()> {
    let mut output_file = File::create(output_path).context("Failed to create output file")?;

    for file_path in files {
        // Read file content
        let content = fs::read_to_string(file_path).with_context(|| {
            format!("Failed to read file: {}", file_path.to_string_lossy())
        })?;

        // Write file path as a comment
        writeln!(
            output_file,
            "\n// ===== FILE: {} =====\n",
            file_path.to_string_lossy()
        )?;

        // Write file content
        writeln!(output_file, "{}", content)?;
    }

    Ok(())
}