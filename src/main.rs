use std::{ env, process };

use taro::*;

fn main() {
  let args: Vec<String> = env::args().collect();

  let config = Config::new(&args).unwrap_or_else(|err| {
    eprintln!("Problem parsing arguments: {err}");
    process::exit(1);
  });

  if let Err (e) = match config.function {
      Functions::Grep => {
        let grep_config = GrepConfig::build(&args).unwrap_or_else(|err| {
          eprintln!("Problem parsing arguments: {err}");
          process::exit(1);
        });
        taro::grep::run_grep(grep_config)
      },
      Functions::Find => {
        let find_config = FindConfig::build(&args).unwrap_or_else(|err| {
          eprintln!("Problem parsing arguments: {err}");
          process::exit(1);
        });
        taro::find::run_find(find_config)
      }
  } {
    eprintln!("Application error: {e}");
    process::exit(1); 
  }
}
