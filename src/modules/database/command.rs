use super::call::CallType;
use super::data::DataType;
use super::database;
use super::database::Error;
use super::path::PathType;
use std::fs::File;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum Command {
    None,
    Get(PathType),
    Call(PathType, CallType),
    Set(PathType, Values),
    Command(PathType, CallType, Values),
}

#[derive(Debug, Clone)]
pub enum Values {
    Data(DataType),
    Path(PathType),
    Call(PathType, CallType),
}

impl Command {
    pub fn execute(&self) -> Result<DataType, Error> {
        let command = self.clone();
        let result = match command {
            Command::None => DataType::None,
            Command::Get(p) => get(p)?,
            Command::Call(p, c) => call(p, c)?,
            Command::Set(p0, v) => match v {
                Values::Data(d) => set_data(p0, d)?,
                Values::Path(p1) => set_path(p0, p1)?,
                Values::Call(p1, c) => set_call(p0, p1, c)?,
            },
            Command::Command(p0, c0, v) => match v {
                Values::Data(d) => command_data(p0, c0, d)?,
                Values::Path(p1) => command_path(p0, c0, p1)?,
                Values::Call(p1, c1) => command_call(p0, c0, p1, c1)?,
            },
        };
        Ok(result)
    }
}

fn get(path: PathType) -> Result<DataType, Error> {
    todo!()
}

fn call(path: PathType, call: CallType) -> Result<DataType, Error> {
    todo!()
}

fn set_data(path: PathType, data: DataType) -> Result<DataType, Error> {
    todo!()
}

fn set_path(path0: PathType, path1: PathType) -> Result<DataType, Error> {
    todo!()
}

fn set_call(path0: PathType, path1: PathType, call: CallType) -> Result<DataType, Error> {
    todo!()
}

fn command_data(path: PathType, call: CallType, data: DataType) -> Result<DataType, Error> {
    todo!()
}

fn command_path(path0: PathType, call: CallType, path1: PathType) -> Result<DataType, Error> {
    todo!()
}

fn command_call(
    path0: PathType,
    call0: CallType,
    path1: PathType,
    call1: CallType,
) -> Result<DataType, Error> {
    todo!()
}
