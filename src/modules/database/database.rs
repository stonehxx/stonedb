use super::error::Error;
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
    let mut data_f = File::create_new(path.join("00000000.DATA"))?;
    create_f.write_all("00000001".as_bytes())?;
    data_f.write_all(&[0u8])?;
    Ok(())
}

pub fn delete(name: impl AsRef<Path>) -> Result<(), Error> {
    let path = name.as_ref();
    if !path.exists() {
        return Err(Error::DatabaseNotFound);
    }
    if !path.join("lock").is_file() {
        return Ok(fs::remove_dir_all(path)?);
    };
    if let Ok(lock) = File::open(path.join("lock")) {
        match lock.try_lock() {
            Ok(()) => (),
            Err(fs::TryLockError::WouldBlock) => return Err(Error::DatabaseInUse),
            Err(fs::TryLockError::Error(e)) => return Err(Error::IoError(e)),
        }
    }
    Ok(fs::remove_dir_all(path)?)
}

pub fn exists(name: impl AsRef<Path>) -> bool {
    let path = name.as_ref();
    path.is_dir()
        && path.join("File").is_dir()
        && path.join("lock").is_file()
        && path.join("create").is_file()
        && path.join("delete").is_file()
        && path.join("config").is_file()
        && path.join("00000000.DATA").is_file()
}

pub fn init() -> Result<(), Error> {
    if exists("system") {
        return Ok(());
    }
    if Path::new("system").exists() {
        delete("syatem")?
    }
    create("system")?;
    todo!();
    Ok(())
}
