#[derive(Debug, Eq, PartialEq)]
pub enum Type {
    Void,
    Int,
    Str,
    Bool,
    Struct(String),
    HigherOrderFunction(Box<Type>, Box<Type>),
    ListInt,
    ListStr,
    ListBool,
    ListStruct(String),
}
