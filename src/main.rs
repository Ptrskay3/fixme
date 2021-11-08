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

fn count_todos_with_ignore() {
    let count = std::sync::atomic::AtomicUsize::new(0);
    let walker = ignore::WalkBuilder::new("/home/leehpeter/mozaweb/js")
        .build_parallel()
        .run(|| {
            Box::new(|res| {
                count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                // println!("{:?}", res);
                return ignore::WalkState::Continue;
            })
        });
    println!("{:?}", count.load(std::sync::atomic::Ordering::Relaxed));
}

fn count_todos() -> usize {
    let count = std::sync::atomic::AtomicUsize::new(0);
    WalkDir::new("/home/leehpeter/mozaweb/")
        .parallelism(Parallelism::RayonNewPool(4))
        .into_iter()
        .par_bridge()
        .for_each(|dir_entry_result| {
            let dir_entry = dir_entry_result.unwrap();
            if dir_entry.file_type().is_file()
                && (dir_entry.path().extension().and_then(OsStr::to_str) == Some("js"))
            {
                let path = dir_entry.path();
                let text = std::fs::read_to_string(&path).unwrap_or_default();
                for (i, line) in text.lines().enumerate() {
                    if RE.find(line).is_some() {
                        count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                        println!("{}:{}:{}", &path.display(), i + 1, 0);
                        println!("{}", &line.red());
                    }
                }
            }
        });
    count.load(std::sync::atomic::Ordering::Relaxed)
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

    println!("Fixme count {:?}", count_todos());
    count_todos_with_ignore();
}
