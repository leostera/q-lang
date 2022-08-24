#[derive(Debug, PartialEq)]
pub struct Id(pub String);

#[derive(Debug, PartialEq)]
pub enum Pattern {}

#[derive(Debug, PartialEq)]
pub struct FunClause {
    pub args: Vec<Pattern>,
    pub body: Expression,
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    LiteralString(String),
    Call { id: Id, args: Vec<Expression> },
    Function(Vec<FunClause>),
}

#[derive(Debug, PartialEq)]
pub struct ValueDeclaration {
    pub name: Id,
    pub value: Expression,
}

#[derive(Debug, PartialEq)]
pub enum ModuleItem {
    ValueDeclaration(ValueDeclaration),
}

#[derive(Debug)]
pub struct Module {
    pub name: Id,
    pub items: Vec<ModuleItem>,
}
