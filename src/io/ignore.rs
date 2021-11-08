#![allow(dead_code)]
use globset::{Glob, GlobSet};
use std::cell::RefCell;
use std::path::PathBuf;
use std::sync::Arc;
use thread_local::ThreadLocal;

pub struct Gitignore {
    set: GlobSet,
    root: PathBuf,
    globs: Vec<Glob>,
    num_ignores: u64,
    num_whitelists: u64,
    matches: Option<Arc<ThreadLocal<RefCell<Vec<usize>>>>>,
}

pub struct Pattern {
    pattern: String,
    is_dir: bool,
}
