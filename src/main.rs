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
        taro::run_grep(grep_config)
      }
  } {
    eprintln!("Application error: {e}");
    process::exit(1); 
  }
}
