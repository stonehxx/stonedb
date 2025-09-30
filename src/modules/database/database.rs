use super::error::Error;
use fslock::LockFile;
use std::{
    fs::{self, File},
    path::Path,
};

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
