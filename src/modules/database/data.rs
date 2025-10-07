#[derive(Debug, Clone)]
pub enum DataType {
    None,
    New(DataNew),
    From(DataFrom),
}

#[derive(Debug, Clone)]
pub enum DataNew {
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
    FN(String),
}

#[derive(Debug, Clone)]
pub enum DataFrom {
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
    Fn(String),
    Date(String),
    Complex(String),
    Chars(String),
}
