pub enum Exp {
    Var(Box<Var>),
    Int(i32),
    Str(String),
    Bool(bool),

    Negate(Box<Exp>),
    Op(Box<Op>),

    // the hard stuff
    Struct(StructDef),
    List(ListDef),
    HigherOrderFunction(Box<HOF>),
}

pub struct StructDef {
    name: String,
    field: Vec<Var>,
}

pub struct ListDef {
    list_type: Type,
    item: Vec<Exp>,
}

pub struct HOF {
    param: Var,
    output: Exp,
}

pub struct Var {
    name: String,
    var_type: Type,
    value: Option<Exp>,
}

pub struct Op {
    op: Operation,
    left: Exp,
    right: Exp,
}
pub enum Operation {
    Add,
    Subtract,
    Divide,
    Multiply,
    Modulo,

    And,
    Or,

    GreaterThan,
    LessThan,
    GreaterEqual,
    LessEqual,
    Equal,
    NotEqual,
}
