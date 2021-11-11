use colored::Colorize;
use git2::Repository;
use ignore::DirEntry;
use indicatif::ProgressIterator;
use lazy_static::lazy_static;
use regex::{Regex, RegexBuilder};
use std::collections::HashMap;
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

lazy_static! {
    static ref RE: Regex = RegexBuilder::new(r#"(\s*(FIXME)|(TODO)|(XXX)|(HACK))[\s+:]"#)
        .case_insensitive(true)
        .build()
        .expect("Failed to initialize search regex.");
}

fn count_todos_with_ignore() {
    let mut buffer = String::new();
    let path = "/home/leehpeter/";
    let repo = Repository::open(path).unwrap();
    let blame_map: Arc<Mutex<HashMap<String, usize>>> = Arc::new(Mutex::new(HashMap::new()));
    let count = AtomicUsize::new(0);
    let _walker = ignore::WalkBuilder::new(path).build_parallel().run(|| {
        Box::new(|res| {
            let blame_map = blame_map.clone();
            process_file(&res.unwrap(), &count, &blame_map);
            ignore::WalkState::Continue
        })
    });
    let blame_map = Arc::try_unwrap(blame_map).unwrap();
    for (key, value) in blame_map
        .into_inner()
        .unwrap()
        .into_iter()
        .progress()
        .nth(1)
    {
        let rel_path = &key.strip_prefix(path).unwrap();
        println!("we are looking at {:?} with #{}", rel_path, value);
        let sig = repo.blame_file(Path::new(rel_path), None).unwrap();
        buffer.push_str(&format!(
            "it was {:?}\n",
            sig.get_line(value)
                .unwrap()
                .final_signature()
                .name()
                .unwrap()
        ));
    }
    println!("{}", buffer);
}

fn process_file(
    entry: &DirEntry,
    count: &AtomicUsize,
    blame_map: &Arc<Mutex<HashMap<String, usize>>>,
) {
    let path = entry.path();
    let text = std::fs::read_to_string(&path).unwrap_or_default();
    for (i, line) in text.lines().enumerate() {
        if RE.find(line).is_some() {
            println!("found at {:?}", line);
            println!("at path {:?}", path);
            count.fetch_add(1, Ordering::SeqCst);
            blame_map
                .lock()
                .unwrap()
                .insert(path.to_str().unwrap().to_owned(), i + 1);
            // println!("{}:{}:{}", &path.display(), i + 1, 0);
            // println!("{}", &line.red());
        }
    }
}

fn main() {
    println!(
        r#"  ______ _                     
 |  ____(_)                    
 | |__   ___  ___ __ ___   ___ 
 |  __| | \ \/ / '_ ` _ \ / _ \
 | |    | |>  <| | | | | |  __/
 |_|    |_/_/\_\_| |_| |_|\___|"#
    );

    count_todos_with_ignore();
}
