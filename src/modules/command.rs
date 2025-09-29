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
