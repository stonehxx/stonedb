#[derive(Debug)]
pub enum Error {
    DatabaseNotFound,
    DatabaseAlreadyExists,
    DatabaseInUse,
    PathError(PathTypeError),
    IoError(std::io::Error),
    Other(String),
}

#[derive(Debug)]
pub enum PathTypeError {
    EmptyPath,
    FirstElementNotDatabase,
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IoError(e)
    }
}
