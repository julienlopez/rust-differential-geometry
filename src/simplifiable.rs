use crate::expression::{BinaryOperation, BinaryOperationType, Expression, Function, Monomial};

pub trait Simplifiable {
    fn simplify_expression(&self) -> Self;
}

impl Simplifiable for Expression {
    fn simplify_expression(&self) -> Self {
        // println!("simplify_expression({:?}", &expression);
        match do_simplify_expression(self) {
            Some(exp) => exp.simplify_expression(),
            None => self.clone(),
        }
    }
}

fn simplify_function_subexpression(
    function: Function,
    expression: &Expression,
) -> Option<Expression> {
    do_simplify_expression(&expression).map(|exp| Expression::Function {
        function: function,
        expression: Box::new(exp),
    })
}

fn simplify_function(function: &Function, expression: &Expression) -> Option<Expression> {
    println!("simplify_function({:?}, {:?})", function, &expression);
    match function {
        Function::Cosine => simplify_function_subexpression(*function, expression),
        Function::Sine => {
            if *expression == Expression::Constant(0.) {
                Some(Expression::Constant(0.))
            } else {
                simplify_function_subexpression(*function, expression)
            }
        }
    }
}

fn simplify_operation(operation: &BinaryOperation) -> Option<Expression> {
    println!(
        "simplify_operation({:?}, {:?}, {:?})",
        operation, &operation.left_value, &operation.right_value
    );
    if *operation.left_value == operation.operation.identity_element() {
        return Some(*operation.right_value.clone());
    }
    if *operation.right_value == operation.operation.identity_element() {
        return Some(*operation.left_value.clone());
    }
    if operation.operation == BinaryOperationType::Multiplication
        && (*operation.left_value == Expression::Constant(0.)
            || *operation.right_value == Expression::Constant(0.))
    {
        return Some(Expression::Constant(0.));
    }
    if operation.operation == BinaryOperationType::Division
        && *operation.left_value == Expression::Constant(0.)
    {
        return Some(Expression::Constant(0.));
    }
    if operation.operation == BinaryOperationType::Addition {
        match (&*operation.left_value, &*operation.right_value) {
            (Expression::Monomial(m1), Expression::Monomial(m2)) => {
                if m1.variable == m2.variable && m1.power == m2.power {
                    return Some(Expression::Monomial(Monomial {
                        factor: m1.factor + m2.factor,
                        variable: m1.variable,
                        power: m1.power,
                    }));
                }
            }
            (_, _) => {}
        }
    }
    if operation.operation == BinaryOperationType::Multiplication {
        match (&*operation.left_value, &*operation.right_value) {
            (Expression::Monomial(m), Expression::Constant(constant))
            | (Expression::Constant(constant), Expression::Monomial(m)) => {
                return Some(Expression::Monomial(Monomial {
                    factor: m.factor * constant,
                    variable: m.variable,
                    power: m.power,
                }))
            }
            (_, _) => {}
        }
    }
    simplify_operation_operands(operation)
}

fn simplify_operation_operands(operation: &BinaryOperation) -> Option<Expression> {
    match (
        do_simplify_expression(&*operation.left_value),
        do_simplify_expression(&*operation.right_value),
    ) {
        (Some(left), Some(right)) => Some(Expression::BinaryOperation(BinaryOperation {
            operation: operation.operation,
            left_value: Box::new(left),
            right_value: Box::new(right),
        })),
        (Some(left), None) => Some(Expression::BinaryOperation(BinaryOperation {
            operation: operation.operation,
            left_value: Box::new(left),
            right_value: Box::new(*operation.right_value.clone()),
        })),
        (None, Some(right)) => Some(Expression::BinaryOperation(BinaryOperation {
            operation: operation.operation,
            left_value: Box::new(*operation.left_value.clone()),
            right_value: Box::new(right),
        })),
        (None, None) => None,
    }
}

fn do_simplify_expression(expr: &Expression) -> Option<Expression> {
    match expr {
        Expression::Function {
            function,
            expression,
        } => simplify_function(function, &*expression),
        Expression::BinaryOperation(operation) => simplify_operation(operation),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simplifiable::*;

    #[test]
    fn test_simplify_expression() {
        let expr = Expression::Constant(5.);
        assert_eq!(expr.simplify_expression(), expr);

        let expr = Expression::Monomial(Monomial {
            factor: 5.,
            variable: 'x',
            power: 2,
        });
        assert_eq!(expr.simplify_expression(), expr);

        let expr = Expression::Function {
            function: Function::Cosine,
            expression: Box::new(Expression::Constant(5.)),
        };
        assert_eq!(expr.simplify_expression(), expr);
        assert_eq!(
            Expression::Function {
                function: Function::Sine,
                expression: Box::new(Expression::Constant(0.))
            }
            .simplify_expression(),
            Expression::Constant(0.)
        );

        let value = Expression::Constant(5.);
        let expr = Expression::BinaryOperation(BinaryOperation {
            operation: BinaryOperationType::Addition,
            left_value: Box::new(value.clone()),
            right_value: Box::new(Expression::Constant(0.)),
        });
        assert_eq!(expr.simplify_expression(), value);

        let expr = Expression::BinaryOperation(BinaryOperation {
            operation: BinaryOperationType::Multiplication,
            left_value: Box::new(value.clone()),
            right_value: Box::new(Expression::Constant(1.)),
        });
        assert_eq!(expr.simplify_expression(), value);

        let expr = Expression::BinaryOperation(BinaryOperation {
            operation: BinaryOperationType::Multiplication,
            left_value: Box::new(value.clone()),
            right_value: Box::new(Expression::Constant(0.)),
        });
        assert_eq!(expr.simplify_expression(), Expression::Constant(0.));
    }

    #[test]
    fn test_simplify_addition_between_two_monomial_of_the_same_order() {
        let expr = Expression::BinaryOperation(BinaryOperation {
            operation: BinaryOperationType::Addition,
            left_value: Box::new(Expression::Monomial(Monomial {
                factor: 5.,
                variable: 'x',
                power: 1,
            })),
            right_value: Box::new(Expression::Monomial(Monomial {
                factor: 3.,
                variable: 'x',
                power: 1,
            })),
        });
        assert_eq!(
            expr.simplify_expression(),
            Expression::Monomial(Monomial {
                factor: 8.,
                variable: 'x',
                power: 1,
            })
        );
    }

    #[test]
    fn test_simplify_multiplication_between_a_constant_and_a_monomial() {
        assert_eq!(
            Expression::BinaryOperation(BinaryOperation {
                operation: BinaryOperationType::Multiplication,
                left_value: Box::new(Expression::Constant(5.)),
                right_value: Box::new(Expression::Monomial(Monomial {
                    factor: 3.,
                    variable: 'x',
                    power: 2
                }))
            })
            .simplify_expression(),
            Expression::Monomial(Monomial {
                factor: 15.,
                variable: 'x',
                power: 2
            })
        );

        assert_eq!(
            Expression::BinaryOperation(BinaryOperation {
                operation: BinaryOperationType::Multiplication,
                left_value: Box::new(Expression::Monomial(Monomial {
                    factor: 3.,
                    variable: 'x',
                    power: 2
                })),
                right_value: Box::new(Expression::Constant(5.)),
            })
            .simplify_expression(),
            Expression::Monomial(Monomial {
                factor: 15.,
                variable: 'x',
                power: 2
            })
        );
    }
}
