use std::{ env, fs, error::Error };

mod modules;
pub use crate::modules::grep;

pub enum Functions {
  Grep
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
      _ => Err("unknown function")
    }
  }
}

pub struct GrepConfig {
  pub query: String,
  pub file_path: String,
  pub ignore_case: bool,
}

impl GrepConfig {
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

pub fn run_grep(config: GrepConfig) -> Result<(), Box<dyn Error>> {
  let contents = fs::read_to_string(config.file_path)?;

  let results = if config.ignore_case {
    grep::search_case_insensitive(&config.query, &contents)
  } else {
    grep::search(&config.query, &contents)
  };

  for line in results {
    println!("{line}");
  }

  Ok(()) 
}
