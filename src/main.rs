use colored::Colorize;
use jwalk::{Parallelism, WalkDir};
use lazy_static::lazy_static;
use rayon::prelude::*;
use regex::{Regex, RegexBuilder};
use std::ffi::OsStr;

lazy_static! {
    static ref RE: Regex = RegexBuilder::new(r#"(\s*(FIXME)|(TODO)|(XXX)|(HACK))[\s+:]"#)
        .case_insensitive(true)
        .build()
        .expect("Failed to initialize search regex.");
}

fn count_todos() -> usize {
    WalkDir::new("/home/leehpeter/mozaweb/js/scripts/mbooklet")
        .parallelism(Parallelism::RayonNewPool(4))
        .into_iter()
        .par_bridge()
        .filter_map(|dir_entry_result| {
            let dir_entry = dir_entry_result.ok()?;
            if dir_entry.file_type().is_file()
                && dir_entry.path().extension().and_then(OsStr::to_str) == Some("js")
            {
                let path = dir_entry.path();
                let text = std::fs::read_to_string(&path).ok()?;
                for (i, line) in text.lines().enumerate() {
                    if let Some(_) = RE.find(&line) {
                        println!("{}:{}:{}", &path.display(), i + 1, 0);
                        println!("{}", &line.red());
                        return Some(true);
                    }
                }
            }
            None
        })
        .count()
}

fn main() {
    println!(
        r#"
_/\\\\\\\\\\\\\\\_______________________________________________________        
\/\\\///////////________________________________________________________       
 \/\\\______________/\\\_________________________________________________      
  \/\\\\\\\\\\\_____\///___/\\\____/\\\____/\\\\\__/\\\\\_______/\\\\\\\\_     
   \/\\\///////_______/\\\_\///\\\/\\\/___/\\\///\\\\\///\\\___/\\\/////\\\    
    \/\\\_____________\/\\\___\///\\\/____\/\\\_\//\\\__\/\\\__/\\\\\\\\\\\_   
     \/\\\_____________\/\\\____/\\\/\\\___\/\\\__\/\\\__\/\\\_\//\\///////__  
      \/\\\_____________\/\\\__/\\\/\///\\\_\/\\\__\/\\\__\/\\\__\//\\\\\\\\\\ 
       \///______________\///__\///____\///__\///___\///___\///____\//////////_"#
    );

    println!("{:?}", count_todos());
}
