use serde::Deserialize;

#[derive(Clone, Debug, Deserialize, Default)]
pub struct Location {
    pub start: usize,
    pub end: usize,
    pub filename: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Text {
    pub text: String,
    pub location: Location,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Let {
    pub name: Variable,
    pub value: Box<Term>,
    pub location: Location,
    pub next: Box<Term>,
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
    pub parameters: Vec<Variable>,
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
    pub value: i32,
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

impl From<rinha::ast::Location> for Location {
    fn from(value: rinha::ast::Location) -> Self {
        Self {
            start: value.start,
            end: value.end,
            filename: value.filename,
        }
    }
}

impl From<rinha::ast::Int> for Integer {
    fn from(value: rinha::ast::Int) -> Self {
        Self {
            value: value.value,
            location: value.location.into(),
        }
    }
}

impl From<rinha::ast::BinaryOp> for BinaryOperator {
    fn from(value: rinha::ast::BinaryOp) -> Self {
        match value {
            rinha::ast::BinaryOp::Eq => BinaryOperator::Eq,
            rinha::ast::BinaryOp::Lt => BinaryOperator::Lt,
            rinha::ast::BinaryOp::Or => BinaryOperator::Or,
            rinha::ast::BinaryOp::Gt => BinaryOperator::Gt,
            rinha::ast::BinaryOp::Add => BinaryOperator::Add,
            rinha::ast::BinaryOp::Sub => BinaryOperator::Sub,
            rinha::ast::BinaryOp::Mul => BinaryOperator::Mul,
            rinha::ast::BinaryOp::Div => BinaryOperator::Div,
            rinha::ast::BinaryOp::Rem => BinaryOperator::Rem,
            rinha::ast::BinaryOp::Neq => BinaryOperator::Neq,
            rinha::ast::BinaryOp::Lte => BinaryOperator::Lte,
            rinha::ast::BinaryOp::Gte => BinaryOperator::Gte,
            rinha::ast::BinaryOp::And => BinaryOperator::And,
        }
    }
}

impl From<rinha::ast::Binary> for Binary {
    fn from(value: rinha::ast::Binary) -> Self {
        Self {
            lhs: value.lhs.into(),
            op: value.op.into(),
            rhs: value.rhs.into(),
            location: value.location.into(),
        }
    }
}

impl From<rinha::ast::Str> for Str {
    fn from(value: rinha::ast::Str) -> Self {
        Self {
            value: value.value,
            location: value.location.into(),
        }
    }
}

impl From<rinha::ast::Bool> for Boolean {
    fn from(value: rinha::ast::Bool) -> Self {
        Self {
            value: value.value,
            location: value.location.into(),
        }
    }
}

impl From<rinha::ast::Tuple> for Tuple {
    fn from(value: rinha::ast::Tuple) -> Self {
        Self {
            first: value.first.into(),
            second: value.second.into(),
            location: value.location.into(),
        }
    }
}

impl From<rinha::ast::Call> for Call {
    fn from(value: rinha::ast::Call) -> Self {
        Self {
            arguments: value
                .arguments
                .iter()
                .cloned()
                .map(Term::from)
                .map(Box::new)
                .collect(),
            callee: value.callee.into(),
            location: value.location.into(),
        }
    }
}

impl From<rinha::ast::Function> for Function {
    fn from(value: rinha::ast::Function) -> Self {
        Self {
            parameters: value
                .parameters
                .iter()
                .cloned()
                .map(Variable::from)
                .collect(),
            value: value.value.into(),
            location: value.location.into(),
        }
    }
}

impl From<rinha::ast::Let> for Let {
    fn from(value: rinha::ast::Let) -> Self {
        Self {
            name: value.name.into(),
            value: value.value.into(),
            location: value.location.into(),
            next: value.next.into(),
        }
    }
}

impl From<rinha::ast::If> for If {
    fn from(value: rinha::ast::If) -> Self {
        Self {
            condition: value.condition.into(),
            then: value.then.into(),
            otherwise: value.otherwise.into(),
            location: value.location.into(),
        }
    }
}

impl From<rinha::ast::Print> for Print {
    fn from(value: rinha::ast::Print) -> Self {
        Self {
            value: value.value.into(),
            location: value.location.into(),
        }
    }
}

impl From<rinha::ast::First> for First {
    fn from(value: rinha::ast::First) -> Self {
        Self {
            value: value.value.into(),
            location: value.location.into(),
        }
    }
}

impl From<rinha::ast::Second> for Second {
    fn from(value: rinha::ast::Second) -> Self {
        Self {
            value: value.value.into(),
            location: value.location.into(),
        }
    }
}

impl From<rinha::parser::Var> for Variable {
    fn from(value: rinha::parser::Var) -> Self {
        Self {
            text: value.text,
            location: value.location.into(),
        }
    }
}

impl From<rinha::ast::Term> for Term {
    fn from(value: rinha::ast::Term) -> Self {
        match value {
            rinha::ast::Term::Error(..) => unimplemented!(),
            rinha::ast::Term::Int(t) => Term::Int(t.into()),
            rinha::ast::Term::Str(t) => Term::Str(t.into()),
            rinha::ast::Term::Call(t) => Term::Call(t.into()),
            rinha::ast::Term::Binary(t) => Term::Binary(t.into()),
            rinha::ast::Term::Function(t) => Term::Function(t.into()),
            rinha::ast::Term::Let(t) => Term::Let(t.into()),
            rinha::ast::Term::If(t) => Term::If(t.into()),
            rinha::ast::Term::Print(t) => Term::Print(t.into()),
            rinha::ast::Term::First(t) => Term::First(t.into()),
            rinha::ast::Term::Second(t) => Term::Second(t.into()),
            rinha::ast::Term::Bool(t) => Term::Bool(t.into()),
            rinha::ast::Term::Tuple(t) => Term::Tuple(t.into()),
            rinha::ast::Term::Var(t) => Term::Var(t.into()),
        }
    }
}

impl From<Box<rinha::ast::Term>> for Box<Term> {
    fn from(value: Box<rinha::ast::Term>) -> Self {
        Box::new(Term::from(*value))
    }
}

impl From<rinha::ast::File> for File {
    fn from(value: rinha::ast::File) -> Self {
        Self {
            name: value.name,
            expression: value.expression.into(),
            location: value.location.into(),
        }
    }
}
