#[derive(Debug)]
pub enum Error {
    DatabaseNotFound,
    DatabaseAlreadyExists,
    DatabaseInUse,
    IoError(std::io::Error),
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IoError(e)
    }
}
