#[derive(Debug, PartialEq)]
pub struct Id(pub String);

#[derive(Debug, PartialEq)]
pub enum Expression {
    LiteralString(String),
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
