enum Expression {
    UIntLiteral(u64),
    Variable(String)
}

pub struct Declaration {
    name: String,
    expression: Expression
}