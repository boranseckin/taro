use std::{ env, path::Path };

mod modules;
pub use crate::modules::grep;
pub use crate::modules::find;

pub enum Functions {
  Grep,
  Find,
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

pub struct FindConfig<'a> {
  pub path: &'a Path,
  pub query: String,
  pub depth: usize,
}

impl<'a> FindConfig<'a> {
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
