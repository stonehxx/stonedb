#[derive(Debug, Clone)]
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