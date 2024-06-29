pub type Variable = char;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum BinaryOperation {
    Addition,
    Substraction,
    Multiplication,
    Division,
}

fn identity_element(operation: BinaryOperation) -> Expression {
    match operation {
        BinaryOperation::Addition | BinaryOperation::Substraction => Expression::Constant(0.),
        BinaryOperation::Multiplication | BinaryOperation::Division => Expression::Constant(1.),
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Function {
    Sine,
    Cosine,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Constant(f64),
    Monomial {
        factor: f64,
        variable: Variable,
        power: u8,
    },
    Operation {
        operation: BinaryOperation,
        left_value: Box<Expression>,
        right_value: Box<Expression>,
    },
    Function {
        function: Function,
        expression: Box<Expression>,
    },
}

fn derive_monomial(
    derivation_variable: Variable,
    factor: &f64,
    variable: &Variable,
    power: &u8,
) -> Expression {
    if *variable == derivation_variable {
        if *power == 1 {
            Expression::Constant(*factor)
        } else {
            Expression::Monomial {
                factor: *factor * (*power as f64),
                variable: *variable,
                power: *power - 1,
            }
        }
    } else {
        Expression::Constant(0.)
    }
}

fn derive_operation(
    derivation_variable: Variable,
    operation: BinaryOperation,
    left_value: &Box<Expression>,
    right_value: &Box<Expression>,
) -> Expression {
    match operation {
        BinaryOperation::Addition | BinaryOperation::Substraction => Expression::Operation {
            operation: operation,
            left_value: Box::new(derive(&left_value, derivation_variable)),
            right_value: Box::new(derive(&right_value, derivation_variable)),
        },
        _ => unimplemented!(),
    }
}

pub fn derive(expression: &Expression, derivation_variable: Variable) -> Expression {
    match expression {
        Expression::Constant(_) => Expression::Constant(0.),
        Expression::Monomial {
            factor,
            variable,
            power,
        } => simplify_expression(derive_monomial(
            derivation_variable,
            factor,
            variable,
            power,
        )),
        Expression::Operation {
            operation,
            left_value,
            right_value,
        } => simplify_expression(derive_operation(
            derivation_variable,
            *operation,
            left_value,
            right_value,
        )),
        _ => unimplemented!(),
    }
}

fn simplify_function(function: Function, expression: Expression) -> Expression {
    match function {
        Function::Cosine => Expression::Function {
            function,
            expression: Box::new(expression),
        },
        Function::Sine => {
            if expression == Expression::Constant(0.) {
                Expression::Constant(0.)
            } else {
                Expression::Function {
                    function,
                    expression: Box::new(expression),
                }
            }
        }
    }
}

fn simplify_operation(
    operation: BinaryOperation,
    left_value: Expression,
    right_value: Expression,
) -> Expression {
    if left_value == identity_element(operation) {
        return right_value;
    }
    if right_value == identity_element(operation) {
        return left_value;
    }
    Expression::Operation {
        operation,
        left_value: Box::new(left_value),
        right_value: Box::new(right_value),
    }
}

fn simplify_expression(expression: Expression) -> Expression {
    match expression {
        Expression::Function {
            function,
            expression,
        } => simplify_function(function, *expression),
        Expression::Operation {
            operation,
            left_value,
            right_value,
        } => simplify_operation(operation, *left_value, *right_value),
        _ => expression,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn derive_constant() {
        assert_eq!(
            derive(&Expression::Constant(5.), 'x'),
            Expression::Constant(0.)
        );
    }

    #[test]
    fn derive_monomial() {
        let x_monomial = Expression::Monomial {
            factor: 5.,
            variable: 'x',
            power: 1,
        };
        assert_eq!(derive(&x_monomial, 'y'), Expression::Constant(0.));
        assert_eq!(derive(&x_monomial, 'x'), Expression::Constant(5.));

        let x_squared_monomial = Expression::Monomial {
            factor: 3.,
            variable: 'x',
            power: 2,
        };
        assert_eq!(derive(&x_squared_monomial, 'y'), Expression::Constant(0.));
        assert_eq!(
            derive(&x_squared_monomial, 'x'),
            Expression::Monomial {
                factor: 6.,
                variable: 'x',
                power: 1
            }
        );

        let x_3_monomial = Expression::Monomial {
            factor: 5.,
            variable: 'x',
            power: 3,
        };
        assert_eq!(derive(&x_3_monomial, 'y'), Expression::Constant(0.));
        assert_eq!(
            derive(&x_3_monomial, 'x'),
            Expression::Monomial {
                factor: 15.,
                variable: 'x',
                power: 2
            }
        );
    }

    #[test]
    fn derive_sum() {
        let left_monomial = Expression::Monomial {
            factor: 5.,
            variable: 'x',
            power: 1,
        };
        let right_monomial = Expression::Monomial {
            factor: 3.,
            variable: 'x',
            power: 2,
        };
        let sum = Expression::Operation {
            operation: BinaryOperation::Addition,
            left_value: Box::new(left_monomial.clone()),
            right_value: Box::new(right_monomial.clone()),
        };

        assert_eq!(derive(&sum, 'y'), Expression::Constant(0.));
        assert_eq!(
            derive(&sum, 'x'),
            Expression::Operation {
                operation: BinaryOperation::Addition,
                left_value: Box::new(Expression::Constant(5.)),
                right_value: Box::new(Expression::Monomial {
                    factor: 6.,
                    variable: 'x',
                    power: 1
                })
            }
        )
    }

    #[test]
    fn test_simplify_expression() {
        let expr = Expression::Constant(5.);
        assert_eq!(simplify_expression(expr.clone()), expr);

        let expr = Expression::Monomial {
            factor: 5.,
            variable: 'x',
            power: 2,
        };
        assert_eq!(simplify_expression(expr.clone()), expr);

        let expr = Expression::Function {
            function: Function::Cosine,
            expression: Box::new(Expression::Constant(5.)),
        };
        assert_eq!(simplify_expression(expr.clone()), expr);
        assert_eq!(
            simplify_expression(Expression::Function {
                function: Function::Sine,
                expression: Box::new(Expression::Constant(0.))
            }),
            Expression::Constant(0.)
        );

        let value = Expression::Constant(5.);
        let expr = Expression::Operation {
            operation: BinaryOperation::Addition,
            left_value: Box::new(value.clone()),
            right_value: Box::new(Expression::Constant(0.)),
        };
        assert_eq!(simplify_expression(expr), value);
    }
}
