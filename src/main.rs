use fixme::io::get_files;
use lazy_static::lazy_static;
use regex::{Regex, RegexBuilder};
use std::fs;

lazy_static! {
    static ref RE: Regex = RegexBuilder::new("fixme")
        .case_insensitive(true)
        .build()
        .expect("Failed to initialize search regex.");
}

fn main() {
    println!(r#"
    _/\\\\\\\\\\\\\\\_______________________________________________________        
    \/\\\///////////________________________________________________________       
     \/\\\______________/\\\_________________________________________________      
      \/\\\\\\\\\\\_____\///___/\\\____/\\\____/\\\\\__/\\\\\_______/\\\\\\\\_     
       \/\\\///////_______/\\\_\///\\\/\\\/___/\\\///\\\\\///\\\___/\\\/////\\\    
        \/\\\_____________\/\\\___\///\\\/____\/\\\_\//\\\__\/\\\__/\\\\\\\\\\\_   
         \/\\\_____________\/\\\____/\\\/\\\___\/\\\__\/\\\__\/\\\_\//\\///////__  
          \/\\\_____________\/\\\__/\\\/\///\\\_\/\\\__\/\\\__\/\\\__\//\\\\\\\\\\ 
           \///______________\///__\///____\///__\///___\///___\///____\//////////_"#);
    let files = get_files(".").unwrap();

    for file in files {
        let contents = fs::read_to_string(&file).expect("Something went wrong reading the file");
        for line in contents.lines() {
            if let Some(m) = RE.find(&line) {
                println!("{}", line);
                println!("at file {:?}", &file);
                println!("starting btye: {}, end: {}", m.start(), m.end());
            }
        }
    }
}
