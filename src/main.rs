// use colored::Colorize;
// use git2::{BlameOptions, Repository};
use ignore::DirEntry;
// use jwalk::{Parallelism, WalkDir};
use lazy_static::lazy_static;
// use rayon::prelude::*;
use regex::{Regex, RegexBuilder};
use std::collections::HashMap;
// use std::ffi::OsStr;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

lazy_static! {
    static ref RE: Regex = RegexBuilder::new(r#"(\s*(FIXME)|(TODO)|(XXX)|(HACK))[\s+:]"#)
        .case_insensitive(true)
        .build()
        .expect("Failed to initialize search regex.");
}

fn count_todos_with_ignore() {
    // let mut blame_options = BlameOptions::new();
    let path = "/home/leehpeter/projects/PySprint/PySprint";
    // let repo = Repository::open(path).unwrap();
    let blame_map: Arc<Mutex<HashMap<String, usize>>> = Arc::new(Mutex::new(HashMap::new()));
    let count = AtomicUsize::new(0);
    let _walker = ignore::WalkBuilder::new(path).build_parallel().run(|| {
        Box::new(|res| {
            let blame_map = blame_map.clone();
            process_file(&res.unwrap(), &count, &blame_map);
            ignore::WalkState::Continue
        })
    });
    println!("{:#?}", blame_map.lock().unwrap());
    // println!("{:#?}", blame_map.lock().unwrap().len());
    println!("{:?}", count.load(Ordering::Relaxed));
}

// fn count_todos() -> usize {
//     let count = std::sync::atomic::AtomicUsize::new(0);
//     WalkDir::new("/home/leehpeter/mozaweb/js")
//         .parallelism(Parallelism::RayonNewPool(4))
//         .into_iter()
//         .par_bridge()
//         .for_each(|dir_entry_result| {
//             let dir_entry = dir_entry_result.unwrap();
//             if dir_entry.file_type().is_file()
//             // && (dir_entry.path().extension().and_then(OsStr::to_str) == Some("js"))
//             {
//                 let path = dir_entry.path();
//                 let text = std::fs::read_to_string(&path).unwrap_or_default();
//                 for (i, line) in text.lines().enumerate() {
//                     if RE.find(line).is_some() {
//                         count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
//                         // println!("{}:{}:{}", &path.display(), i + 1, 0);
//                         // println!("{}", &line.red());
//                     }
//                 }
//             }
//         });
//     count.load(std::sync::atomic::Ordering::Relaxed)
// }

fn process_file<'a>(
    entry: &'a DirEntry,
    count: &AtomicUsize,
    blame_map: &Arc<Mutex<HashMap<String, usize>>>,
) {
    let path = entry.path();
    let text = std::fs::read_to_string(&path).unwrap_or_default();
    for (i, line) in text.lines().enumerate() {
        if RE.find(line).is_some() {
            count.fetch_add(1, Ordering::SeqCst);
            blame_map
                .lock()
                .unwrap()
                .insert(path.to_str().unwrap().to_owned(), i + 1);
            println!("{}:{}:{}", &path.display(), i + 1, 0);
            // println!("{}", &line.red());
        }
    }
}

fn main() {
    println!(
        r#"_/\\\\\\\\\\\\\\\_______________________________________________________        
\/\\\///////////________________________________________________________       
 \/\\\______________/\\\_________________________________________________      
  \/\\\\\\\\\\\_____\///___/\\\____/\\\____/\\\\\__/\\\\\_______/\\\\\\\\_     
   \/\\\///////_______/\\\_\///\\\/\\\/___/\\\///\\\\\///\\\___/\\\/////\\\    
    \/\\\_____________\/\\\___\///\\\/____\/\\\_\//\\\__\/\\\__/\\\\\\\\\\\_   
     \/\\\_____________\/\\\____/\\\/\\\___\/\\\__\/\\\__\/\\\_\//\\///////__  
      \/\\\_____________\/\\\__/\\\/\///\\\_\/\\\__\/\\\__\/\\\__\//\\\\\\\\\\ 
       \///______________\///__\///____\///__\///___\///___\///____\//////////_"#
    );

    // println!("Fixme count {:?}", count_todos());
    count_todos_with_ignore();
}
