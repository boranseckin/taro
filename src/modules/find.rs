//! Finds all the files recursively in a directory that matches the substring query provided.
//! 
//! The depth (10 by default) of the search tree can be specified.

use std::{ path::Path, fs, error::Error };

use crate::FindConfig;

fn traverse(path: &Path, query: &str, depth: usize, i: usize) -> i32 {
  if i > depth { return 0 };

  let mut found_count = 0;

  match fs::read_dir(path) {
    Err(e) => println!("Cannot read directory {:?}: {}", path.file_name().unwrap(), e),
    Ok(entries) => {
      for entry in entries.flatten() {
        if entry.file_name().to_str().unwrap().contains(query) {
          println!("{}", entry.path().to_str().unwrap());
          found_count += 1;
        }

        if let Ok(metadata) = entry.metadata() {
          if metadata.is_dir() {
            found_count += traverse(&entry.path(), query, depth, i + 1);
          }
        }
      }
    }
  };

  found_count
}

/// Runs the find function using the [FindConfig] provided.
pub fn run_find(config: FindConfig) -> Result<(), Box<dyn Error>> {
  println!("Searching for {} in {:?}", config.query, config.path);

  let found_count = traverse(config.path, config.query.as_str(), config.depth, 0);

  if found_count == 0 {
    println!("No match found.");
  } else {
    println!("Found {} matches.", found_count);
  }

  Ok(())
}
