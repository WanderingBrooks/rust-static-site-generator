extern crate markdown;

use std::env;
use std::fs;
use std::process;
use std::io;

use static_site_generator::Config;


fn main() {
  let args: Vec<String> = env::args().collect();

  let config = Config::new(&args).unwrap_or_else(|err| {
    println!("Problem parsing arguments: {}", err);
    process::exit(1);
  }); 

  println!("In path {}", config.path);

  let files = extract_paths(&config.path).unwrap_or_else(|err| {
      println!("Problem reading directroy: {}", err);
      process::exit(1);
  });

  let result_directory = "html";

  fs::create_dir(&result_directory);

  for file in files {
    let html = markdown::to_html(&file.contents);
    let mut new_path = file.path.clone();
    new_path.replace_range(..config.path.len(), result_directory);

    fs::write(new_path, html);
  }

  // let html : String = markdown::to_html(&contents);

  // println!("With html:\n{}", html);
}

struct File {
  path: String,
  contents: String
}

fn extract_paths(path: &String) -> io::Result<Vec<File>> {
  let mut vec = Vec::new();

  for entry in fs::read_dir(path)? {
    let entry = entry?;
    let sub_path = entry.path();
    let sub_path_string = sub_path.display().to_string();
    println!("{}", sub_path_string);
    if sub_path.is_dir() {
        vec.extend(extract_paths(&sub_path_string)? );
    } else {
        let contents = fs::read_to_string(&sub_path_string)
          .expect("Something went wrong reading the file");

        vec.push(File { path: sub_path_string, contents })
    }
  }

  Ok(vec)
}