use super::error::Error;
use fslock::LockFile;
use std::{
    fs::{self, File},
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub enum Command {
    None,
    Get(Vec<PathType>),
    Set(Vec<PathType>, Set),
}

#[derive(Debug)]
pub enum Set {
    PathType(Vec<PathType>),
    DataType(DataType),
    NewType(NewType),
}

#[derive(Debug)]
pub enum PathType {
    Fn(String),
    Select(String),
    Num(String),
    //Call(CallType, Vec<String>),
    Call(String, String),
    Field(String),
    Database(String),
    Object(String),
}

#[derive(Debug)]
pub enum DataType {
    String(String),
    Array(Vec<String>),
    Object(Vec<(String, String)>),
    Float(String),
    Bytes(String),
    Link(String),
    Number(String),
    Table(String),
    Tuple(String),
    Tenser(String),
    None,
    NaN,
    Fn(String),
    Date(String),
    Complex(String),
}

#[derive(Debug)]
pub enum NewType {
    String(Option<String>),
    Array(Option<String>),
    Object(Option<(String, String)>),
    Float(Option<String>),
    Bytes(Option<String>),
    Link(Option<String>),
    Number(Option<String>),
    Table(Option<Vec<String>>, Option<Vec<String>>),
    Tuple(Option<String>),
    Tenser(String),
    Date(String),
    Enum(String),
    Complex(String),
    Chars(String),
}

#[derive(Debug)]
pub enum CallType {
    Push(),
    Pop(),
    Insert(),
    Get(),
    First(),
    Last(),
    Len(),
    IsEmpty(),
    Remove(),
    Clear(),
    Retain(),
    Truncate(),
    Contains(),
    Dedup(),
}


pub fn create(name: impl AsRef<Path>) -> Result<(), Error> {
    let path = name.as_ref();
    if path.is_dir() {
        return Err(Error::DatabaseAlreadyExists);
    }
    fs::create_dir_all(path)?;
    File::create_new(path.join("main"))?;
    File::create_new(path.join("lock"))?;
    Ok(())
}

pub fn delete(name: impl AsRef<Path>) -> Result<(), Error> {
    let path = name.as_ref();
    if !path.is_dir() {
        return Err(Error::DatabaseNotFound);
    }
    if let Ok(mut lock) = LockFile::open(&path.join("lock")) {
        if !lock.try_lock()? {
            return Err(Error::DatabaseInUse);
        }
    }
    Ok(fs::remove_dir_all(path)?)
}

pub fn exists(name: impl AsRef<Path>) -> bool {
    name.as_ref().is_dir()
}

pub fn init() -> Result<(), Error> {
    if exists("system") {
        return Ok(());
    }
    create("system")?;
    Ok(())
}

pub fn run(command: Command) -> Result<DataType, Error> {
    match command {
        Command::Get(p) => get(p),
        Command::Set(p, v) => set(p, v),
        Command::None => Ok(DataType::None),
    }
}

pub fn set(paths: Vec<PathType>, set: Set) -> Result<DataType, Error> {
    match set {
        Set::DataType(d) => todo!(),
        Set::NewType(d) => todo!(),
        Set::PathType(d) => {
            let db = &paths[0];
            let path = match db {
                PathType::Database(db) => PathBuf::from(db),
                _ => return Err(Error::Other(String::new())),
            };
            let mut file = File::open(path.join("main"))?;
            for i in 1..paths.len() {
                match path {
                    _ => todo!(),
                }
            }
            let data = get(d)?;
            //btree.push(get())
            Ok(DataType::String(String::from("OK")))
        }
    }
}

pub fn get(paths: Vec<PathType>) -> Result<DataType, Error> {
    let db = &paths[0];
    let path = match db {
        PathType::Database(db) => PathBuf::from(db),
        _ => return Err(Error::Other(String::new())),
    };
    let mut file = File::open(path.join("main"))?;
    if paths.len() == 1 {
        return Ok(DataType::String(
            String::from("Database: ") + path.as_os_str().to_str().unwrap(),
        ));
    }
    for i in 1..paths.len() - 1 {
        match path {
            _ => todo!(),
        }
    }
    match paths.last() {
        _ => todo!(),
    };
    Ok(DataType::None)
}
