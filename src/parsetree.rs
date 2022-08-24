#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct Id(pub String);

#[derive(Clone, Debug, PartialEq)]
pub enum Pattern {
    Bind(Id),
}

#[derive(Clone, Debug, PartialEq)]
pub struct FunClause {
    pub args: Vec<Pattern>,
    pub body: Expression,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expression {
    LiteralString(String),
    Call { id: Id, args: Vec<Expression> },
    Function(Vec<FunClause>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ValueDeclaration {
    pub name: Id,
    pub value: Expression,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ModuleItem {
    ValueDeclaration(ValueDeclaration),
}

#[derive(Clone, Debug)]
pub struct Module {
    pub name: Id,
    pub items: Vec<ModuleItem>,
}
