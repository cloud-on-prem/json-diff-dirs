use assert_json_diff::{assert_json_matches_no_panic, CompareMode};
use colored::*;
use serde_json::Value;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::Error;
use std::path::Path;
use walkdir::{DirEntry, WalkDir};

fn is_json(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map_or(false, |s| s.ends_with(".json"))
}

fn compare_json(file1: &Path, file2: &Path) -> Result<(), Error> {
    let f1 = File::open(file1)?;
    let f2 = File::open(file2)?;

    let json1: BTreeMap<String, Value> = serde_json::from_reader(f1)?;
    let json2: BTreeMap<String, Value> = serde_json::from_reader(f2)?;

    let mode = CompareMode::Inclusive;
    let config = assert_json_diff::Config::new(mode);
    let result = assert_json_matches_no_panic(&json1, &json2, config);

    if let Err(e) = result {
        println!(
            "{}",
            format!(
                "Differences in {}:",
                file1.file_name().unwrap().to_str().unwrap()
            )
            .yellow()
        );
        println!("  {} {}", "Difference:".bold(), e.red());
    }

    Ok(())
}

fn main() {
    let dir1 = std::env::args().nth(1).expect("Directory 1 required");
    let dir2 = std::env::args().nth(2).expect("Directory 2 required");

    let walker = WalkDir::new(dir1)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(is_json);

    for entry in walker {
        let json_file = entry.path();
        let file_name = json_file.file_name().unwrap();
        let json_file2 = Path::new(&dir2).join(file_name);

        if json_file2.exists() {
            if let Err(e) = compare_json(json_file, &json_file2) {
                eprintln!(
                    "{} {}",
                    "Error comparing:".red().bold(),
                    format!("{}: {}", json_file.display(), e).red()
                );
            }
        }
    }
}
