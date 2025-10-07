use std::fs::File;
use std::path::PathBuf;

use super::data::DataType;
use super::error::Error;
use super::error::PathTypeError;
use super::new::NewType;
use super::path::PathType;

#[derive(Debug, Clone)]
pub enum Command {
    None,
    Get(Vec<PathType>),
    Set(Vec<PathType>, Value),
}

#[derive(Debug, Clone)]
pub enum Value {
    NewType(NewType),
    DataType(DataType),
    PathType(Vec<PathType>),
}

impl Command {
    pub fn execute(&self) -> Result<DataType, Error> {
        let command = self.clone();
        let result = match command {
            Command::None => DataType::None,
            Command::Get(path) => get(path)?,
            Command::Set(path, set) => match set {
                Value::NewType(i) => new(path, i)?,
                Value::DataType(i) => data(path, i)?,
                Value::PathType(i) => from(path, i)?,
            },
        };
        Ok(result)
    }
}

fn get(path: Vec<PathType>) -> Result<DataType, Error> {
    if path.is_empty() {
        return Err(Error::PathError(PathTypeError::EmptyPath));
    }
    let database_path = match &path[0] {
        PathType::Database(db) => PathBuf::from(db),
        _ => return Err(Error::PathError(PathTypeError::FirstElementNotDatabase)),
    };
    if !database_path.is_dir() {
        return Err(Error::DatabaseNotFound);
    }
    let mut file = File::open(database_path.join("main"))?;
    if path.len() == 1 {
        return Ok(DataType::String(
            String::from("Database: ") + database_path.to_str().unwrap(),
        ));
    }
    for i in 1..path.len() - 1 {
        match path {
            _ => todo!(),
        }
    }
    match path.last() {
        _ => todo!(),
    };
    Ok(DataType::None)
}

fn new(path: Vec<PathType>, new: NewType) -> Result<DataType, Error> {
    todo!()
}

fn data(path: Vec<PathType>, data: DataType) -> Result<DataType, Error> {
    todo!()
}

fn from(path: Vec<PathType>, from: Vec<PathType>) -> Result<DataType, Error> {
    if path.is_empty() {
        return Err(Error::PathError(PathTypeError::EmptyPath));
    }
    let database_path = match &path[0] {
        PathType::Database(db) => PathBuf::from(db),
        _ => return Err(Error::PathError(PathTypeError::FirstElementNotDatabase)),
    };
    if !database_path.is_dir() {
        return Err(Error::DatabaseNotFound);
    }
    let mut file = File::open(database_path.join("main"))?;
    for i in 1..path.len() {
        match path {
            _ => todo!(),
        }
    }
    let data = get(path)?;
    //btree.push(get())
    Ok(DataType::String(String::from("OK")))
}
