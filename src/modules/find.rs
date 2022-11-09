use std::{ path::Path, fs, error::Error };

use crate::FindConfig;

fn traverse(path: &Path, query: &str, depth: usize, i: usize) -> i32 {
  if i > depth { return 0 };

  let mut found_count = 0;

  match fs::read_dir(path) {
    Err(e) => println!("Cannot read directory {:?}: {}", path.file_name().unwrap(), e),
    Ok(entries) => {
      for entry in entries {
        if let Ok(entry) = entry {
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
    }
  };

  return found_count;
}

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
