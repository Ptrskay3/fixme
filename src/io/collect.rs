use std::path::PathBuf;
use std::{fs, io};

pub fn get_files(root: &str) -> io::Result<Vec<PathBuf>> {
    let mut result = vec![];

    for path in fs::read_dir(root)? {
        let path = path?.path();
        if !path.is_dir() {
            result.push(path.to_owned());
        }
    }
    // result.sort();
    Ok(result)
}
