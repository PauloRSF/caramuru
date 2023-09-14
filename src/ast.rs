use serde::Deserialize;

#[derive(Clone, Debug, Deserialize, Default)]
pub struct Location {
    pub start: u32,
    pub end: u32,
    pub filename: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Text {
    pub text: String,
    pub location: Location,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Let {
    pub name: Text,
    pub value: Box<Term>,
    pub location: Location,
    pub next: Option<Box<Term>>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "Var")]
pub struct Variable {
    pub text: String,
    pub location: Location,
}

#[derive(Clone, Debug, Deserialize)]
pub enum BinaryOperator {
    Eq,
    Lt,
    Or,
    Gt,
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Neq,
    Lte,
    Gte,
    And,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Binary {
    pub lhs: Box<Term>,
    pub op: BinaryOperator,
    pub rhs: Box<Term>,
    pub location: Location,
}

#[derive(Clone, Debug, Deserialize)]
pub struct If {
    pub condition: Box<Term>,
    pub then: Box<Term>,
    pub otherwise: Box<Term>,
    pub location: Location,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Function {
    pub parameters: Vec<Text>,
    pub value: Box<Term>,
    pub location: Location,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Call {
    pub callee: Box<Term>,
    pub arguments: Vec<Box<Term>>,
    pub location: Location,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Print {
    pub value: Box<Term>,
    pub location: Location,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Tuple {
    pub first: Box<Term>,
    pub second: Box<Term>,
    pub location: Location,
}

#[derive(Clone, Debug, Deserialize)]
pub struct First {
    pub value: Box<Term>,
    pub location: Location,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Second {
    pub value: Box<Term>,
    pub location: Location,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Boolean {
    pub value: bool,
    pub location: Location,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Str {
    pub value: String,
    pub location: Location,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Integer {
    pub value: u32,
    pub location: Location,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "kind")]
pub enum Term {
    If(If),
    Str(Str),
    Let(Let),
    Call(Call),
    First(First),
    Int(Integer),
    Print(Print),
    Tuple(Tuple),
    Var(Variable),
    Bool(Boolean),
    Second(Second),
    Binary(Binary),
    Function(Function),
}

#[derive(Debug, Deserialize)]
pub struct File {
    pub name: String,
    pub expression: Term,
    pub location: Location,
}
