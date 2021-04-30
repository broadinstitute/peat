pub(crate) enum Expression {
    UIntLiteral(u64),
    BoolLiteral(bool),
    Variable(String)
}

pub(crate) struct Declaration {
    pub(crate) name: String,
    pub(crate) expression: Expression
}