use std::collections::HashSet;

pub type Variable = char;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum BinaryOperationType {
    Addition,
    Substraction,
    Multiplication,
    Division,
}

impl BinaryOperationType {
    pub fn identity_element(&self) -> Expression {
        match self {
            BinaryOperationType::Addition | BinaryOperationType::Substraction => {
                Expression::Constant(0.)
            }
            BinaryOperationType::Multiplication | BinaryOperationType::Division => {
                Expression::Constant(1.)
            }
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Function {
    Sine,
    Cosine,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BinaryOperation {
    pub operation: BinaryOperationType,
    pub left_value: Box<Expression>,
    pub right_value: Box<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Monomial {
    pub factor: f64,
    pub variable: Variable,
    pub power: u8,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Constant(f64),
    NamedConstant(&'static str),
    Monomial(Monomial),
    BinaryOperation(BinaryOperation),
    Function {
        function: Function,
        expression: Box<Expression>,
    },
}

impl Expression {
    pub fn variables(&self) -> HashSet<Variable> {
        match &self {
            Expression::Constant(_) | Expression::NamedConstant(_) => HashSet::<Variable>::new(),
            Expression::Monomial(m) => HashSet::from([m.variable]),
            Expression::Function { expression, .. } => expression.variables(),
            Expression::BinaryOperation(operation) => {
                let mut left_vars = operation.left_value.variables();
                left_vars.extend(operation.right_value.variables());
                left_vars
            }
        }
    }
}
