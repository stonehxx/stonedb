#[derive(Debug, Clone)]
pub enum PathType {
    Fn(String),
    Select(String),
    Num(String),
    //Call(CallType, Vec<String>),
    Call(String, String),
    Field(String),
    Database(String),
    Object(String),
}