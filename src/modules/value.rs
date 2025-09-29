#[derive(Debug)]
pub enum Value {
    Array,
    Object,
    Tuple,
    Table,
    Bytes,
    String,
    Number,
    Float,
    Link,
}
