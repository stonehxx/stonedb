use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct PathType {
    database: PathBuf,
    paths: Vec<PathTypes>,
}

#[derive(Debug, Clone)]
pub enum PathTypes {
    Fn(String),
    Regex(String),
    Range(Option<i64>, Option<i64>),
    String(String),
    Num(u64),
}

#[derive(Debug)]
pub enum PathTypeError {
    EmptyPath,
    FirstElementNotDatabase,
}
