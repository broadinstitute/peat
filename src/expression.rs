pub(crate) enum Expression {
    UIntLiteral(u64),
    BoolLiteral(bool),
    Variable(String)
}

pub struct Declaration {
    name: String,
    expression: Expression
}