//! Wrapper for multiple quality of life command-line tools.
//! 
//! Includes:
//! - [find]
//! - [gitignore]
//! - [grep]

use std::{ str, vec };
use std::{ env, path::Path };

mod modules;
pub use crate::modules::grep;
pub use crate::modules::find;
pub use crate::modules::gitignore;

/// Possible functions that can be run using the command line interface.
pub enum Functions {
  Grep,
  Find,
  GitIgnore
}

pub struct Config {
  pub function: Functions
}

impl Config {
  pub fn new(args: &[String]) -> Result<Config, &'static str> {
    if args.len() < 2 {
      return Err("not enough arguments");
    }

    match args[1].as_str() {
      "grep" => Ok(Config { function: Functions::Grep }),
      "find" => Ok(Config { function: Functions::Find }),
      "gitignore" => Ok(Config { function: Functions::GitIgnore }),
      _ => Err("unknown function")
    }
  }
}

/// Required configuration for [grep].
pub struct GrepConfig {
  /// Substring to query for.
  pub query: String,
  /// Path of the file to query.
  pub file_path: String,
  /// Ignore-case flag.
  pub ignore_case: bool,
}

impl GrepConfig {
  /// Builds the [GrepConfig] from command-line arguments
  pub fn build(args: &[String]) -> Result<GrepConfig, &'static str> {
    if args.len() < 4 {
      return Err("not enough argument for grep");
    }

    let query = args[2].clone();
    let file_path = args[3].clone();

    let ignore_case = env::var("IGNORE_CASE").is_ok();

    Ok(GrepConfig { query, file_path, ignore_case })
  }
}

/// Required configuration for [find].
pub struct FindConfig<'a> {
  /// Path of the folder to start the search.
  pub path: &'a Path,
  /// Substring to query for.
  pub query: String,
  /// Folder depth value for the serach.
  pub depth: usize,
}

impl<'a> FindConfig<'a> {
  /// Builds the [FindConfig] from command-line arguments
  pub fn build(args: &[String]) -> Result<FindConfig, &'static str> {
    if args.len() < 4 {
      return Err("not enough argument for find");
    }

    let path = Path::new(args[2].as_str());
    let query = args[3].clone();

    let depth = match env::var("DEPTH") {
      Ok(depth) => depth.parse().unwrap_or_else(|_| {
        eprintln!("Cannot parse depth \"{}\" as an integer, using default value (10)", depth); 
        10
      }),
      Err(_) => {
        println!("Using default depth value (10)");
        10
      }
    };

    Ok(FindConfig { path, query, depth })
  }
}

/// Required configuration for [gitignore].
pub struct GitIgnoreConfig {
  /// Operating Systems, IDEs, Languages to include in the gitignore.
  pub include: Vec<String>,
  /// Optional output path to save the generated gitignore.
  pub output: Option<String>,
}

impl GitIgnoreConfig {
  /// Builds the [GitIgnoreConfig] from command-line arguments
  pub fn build(args: &[String]) -> Result<GitIgnoreConfig, &'static str> {
    if args.len() < 3 {
      return Err("not enough argument for gitignore");
    }

    let mut include = vec![String::new(); args.len() - 2];

    for (i, item) in args.iter().skip(2).enumerate() {
      include[i] = item.clone();
    }

    let output = env::var("OUTPUT").ok();

    Ok(GitIgnoreConfig { include, output })
  }
}
