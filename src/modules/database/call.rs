
#[derive(Debug, Clone)]
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
