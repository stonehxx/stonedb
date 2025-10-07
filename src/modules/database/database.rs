use super::error::Error;
use fslock::LockFile;
use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

pub fn create(name: impl AsRef<Path>) -> Result<(), Error> {
    let path = name.as_ref();
    if path.exists() {
        return Err(Error::DatabaseAlreadyExists);
    }
    fs::create_dir_all(path)?;
    fs::create_dir(path.join("File"))?;
    File::create_new(path.join("lock"))?;
    let mut create_f = File::create_new(path.join("create"))?;
    File::create_new(path.join("delete"))?;
    File::create_new(path.join("config"))?;
    let mut index_f = File::create_new(path.join("00000000.INDEX"))?;
    File::create_new(path.join("00000000.DATA"))?;
    create_f.write_all("00000001".as_bytes())?;
    index_f.write_all(&[0u8])?;
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
