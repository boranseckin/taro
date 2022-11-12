use std::{ error::Error, fs::File, io::Write };

use crate::GitIgnoreConfig;

const URL: &str = "https://www.gitignore.io/api/";

fn create_url(include: &[String]) -> String {
  let mut url = String::from(URL);
  url += include.join(",").as_str();
  url
}

fn fetch(url: String) -> Result<String, Box<dyn Error>> {
  let res = match reqwest::blocking::get(url) {
    Ok(res) => res.text()?,
    Err(_) => Err("cannot fetch gitignore")?
  };

  Ok(res)
}

pub fn run_gitignore(config: GitIgnoreConfig) -> Result<(), Box<dyn Error>> {
  let url = create_url(&config.include);
 
  let res = fetch(url)?;

  if let Some(output) = config.output {
    let mut file = File::create(output)?;
    file.write_all(res.as_bytes())?;
  } else {
    println!("{}", res);
  }

  Ok(())
}

#[cfg(test)]
mod test {
  use super::{*, create_url};

  #[test]
  fn test_url() {
    let include = ["node", "python", "rust"].map(String::from);
    let expected = String::from(URL) + "node,python,rust";
    assert_eq!(expected, create_url(&include));
  }

  #[test]
  fn test_fetch_endpoint() {
    let url = String::from(URL) + "node,python,rust";

    assert!(fetch(url).is_ok())
  }

  #[test]
  fn test_fetch_result() {
    let url = String::from(URL) + "node,python,rust";
    let res = fetch(url).unwrap();

    assert!(res.contains("node"));
    assert!(res.contains("python"));
    assert!(res.contains("rust"));
  }
}
