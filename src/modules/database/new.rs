#[derive(Debug, Clone)]
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