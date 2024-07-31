use std::collections::HashSet;

pub type Variable = char;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum BinaryOperationType {
    Addition,
    Substraction,
    Multiplication,
    Division,
}

fn identity_element(operation: BinaryOperationType) -> Expression {
    match operation {
        BinaryOperationType::Addition | BinaryOperationType::Substraction => {
            Expression::Constant(0.)
        }
        BinaryOperationType::Multiplication | BinaryOperationType::Division => {
            Expression::Constant(1.)
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

fn derive_monomial(derivation_variable: Variable, monomial: &Monomial) -> Expression {
    if monomial.variable == derivation_variable {
        if monomial.power == 1 {
            Expression::Constant(monomial.factor)
        } else {
            Expression::Monomial(Monomial {
                factor: monomial.factor * (monomial.power as f64),
                variable: monomial.variable,
                power: monomial.power - 1,
            })
        }
    } else {
        Expression::Constant(0.)
    }
}

fn derive_operation(derivation_variable: Variable, operation: &BinaryOperation) -> Expression {
    match operation.operation {
        BinaryOperationType::Addition | BinaryOperationType::Substraction => {
            Expression::BinaryOperation(BinaryOperation {
                operation: operation.operation,
                left_value: Box::new(derive(&operation.left_value, derivation_variable)),
                right_value: Box::new(derive(&operation.right_value, derivation_variable)),
            })
        }
        BinaryOperationType::Multiplication => Expression::BinaryOperation(BinaryOperation {
            operation: BinaryOperationType::Addition,
            left_value: Box::new(Expression::BinaryOperation(BinaryOperation {
                operation: BinaryOperationType::Multiplication,
                left_value: Box::new(derive(&operation.left_value, derivation_variable)),
                right_value: Box::new(*operation.right_value.clone()),
            })),
            right_value: Box::new(Expression::BinaryOperation(BinaryOperation {
                operation: BinaryOperationType::Multiplication,
                left_value: Box::new(derive(&operation.right_value, derivation_variable)),
                right_value: Box::new(*operation.left_value.clone()),
            })),
        }),
        _ => unimplemented!(),
    }
}

pub fn derive(expression: &Expression, derivation_variable: Variable) -> Expression {
    match expression {
        Expression::Constant(_) | Expression::NamedConstant(_) => Expression::Constant(0.),
        Expression::Monomial(m) => simplify_expression(derive_monomial(derivation_variable, m)),
        Expression::BinaryOperation(operation) => {
            simplify_expression(derive_operation(derivation_variable, operation))
        }
        _ => unimplemented!(),
    }
}

fn simplify_function_subexpression(
    function: Function,
    expression: Expression,
) -> Option<Expression> {
    do_simplify_expression(expression).map(|exp| Expression::Function {
        function: function,
        expression: Box::new(exp),
    })
}

fn simplify_function(function: Function, expression: Expression) -> Option<Expression> {
    println!("simplify_function({:?}, {:?})", function, &expression);
    match function {
        Function::Cosine => simplify_function_subexpression(function, expression),
        Function::Sine => {
            if expression == Expression::Constant(0.) {
                Some(Expression::Constant(0.))
            } else {
                simplify_function_subexpression(function, expression)
            }
        }
    }
}

fn simplify_operation(operation: BinaryOperation) -> Option<Expression> {
    println!(
        "simplify_operation({:?}, {:?}, {:?})",
        operation, &operation.left_value, &operation.right_value
    );
    if *operation.left_value == identity_element(operation.operation) {
        return Some(*operation.right_value);
    }
    if *operation.right_value == identity_element(operation.operation) {
        return Some(*operation.left_value);
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

fn simplify_operation_operands(operation: BinaryOperation) -> Option<Expression> {
    match (
        do_simplify_expression(*operation.left_value.clone()),
        do_simplify_expression(*operation.right_value.clone()),
    ) {
        (Some(left), Some(right)) => Some(Expression::BinaryOperation(BinaryOperation {
            operation: operation.operation,
            left_value: Box::new(left),
            right_value: Box::new(right),
        })),
        (Some(left), None) => Some(Expression::BinaryOperation(BinaryOperation {
            operation: operation.operation,
            left_value: Box::new(left),
            right_value: Box::new(*operation.right_value),
        })),
        (None, Some(right)) => Some(Expression::BinaryOperation(BinaryOperation {
            operation: operation.operation,
            left_value: Box::new(*operation.left_value),
            right_value: Box::new(right),
        })),
        (None, None) => None,
    }
}

fn do_simplify_expression(expr: Expression) -> Option<Expression> {
    match expr {
        Expression::Function {
            function,
            expression,
        } => simplify_function(function, *expression),
        Expression::BinaryOperation(operation) => simplify_operation(operation),
        _ => None,
    }
}

fn simplify_expression(expression: Expression) -> Expression {
    println!("simplify_expression({:?}", &expression);
    match do_simplify_expression(expression.clone()) {
        Some(exp) => simplify_expression(exp),
        None => expression,
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn derive_constant() {
        assert_eq!(
            derive(&Expression::Constant(5.), 'x'),
            Expression::Constant(0.)
        );

        assert_eq!(
            derive(&Expression::NamedConstant("pi"), 'x'),
            Expression::Constant(0.)
        );
    }

    #[test]
    fn derive_monomial() {
        let x_monomial = Expression::Monomial(Monomial {
            factor: 5.,
            variable: 'x',
            power: 1,
        });
        assert_eq!(derive(&x_monomial, 'y'), Expression::Constant(0.));
        assert_eq!(derive(&x_monomial, 'x'), Expression::Constant(5.));

        let x_squared_monomial = Expression::Monomial(Monomial {
            factor: 3.,
            variable: 'x',
            power: 2,
        });
        assert_eq!(derive(&x_squared_monomial, 'y'), Expression::Constant(0.));
        assert_eq!(
            derive(&x_squared_monomial, 'x'),
            Expression::Monomial(Monomial {
                factor: 6.,
                variable: 'x',
                power: 1
            })
        );

        let x_3_monomial = Expression::Monomial(Monomial {
            factor: 5.,
            variable: 'x',
            power: 3,
        });
        assert_eq!(derive(&x_3_monomial, 'y'), Expression::Constant(0.));
        assert_eq!(
            derive(&x_3_monomial, 'x'),
            Expression::Monomial(Monomial {
                factor: 15.,
                variable: 'x',
                power: 2
            })
        );
    }

    #[test]
    fn derive_sum() {
        let left_monomial = Expression::Monomial(Monomial {
            factor: 5.,
            variable: 'x',
            power: 1,
        });
        let right_monomial = Expression::Monomial(Monomial {
            factor: 3.,
            variable: 'x',
            power: 2,
        });
        let sum = Expression::BinaryOperation(BinaryOperation {
            operation: BinaryOperationType::Addition,
            left_value: Box::new(left_monomial.clone()),
            right_value: Box::new(right_monomial.clone()),
        });

        assert_eq!(derive(&sum, 'y'), Expression::Constant(0.));
        assert_eq!(
            derive(&sum, 'x'),
            Expression::BinaryOperation(BinaryOperation {
                operation: BinaryOperationType::Addition,
                left_value: Box::new(Expression::Constant(5.)),
                right_value: Box::new(Expression::Monomial(Monomial {
                    factor: 6.,
                    variable: 'x',
                    power: 1
                }))
            })
        )
    }

    #[test]
    fn derive_product() {
        let left_monomial = Expression::Monomial(Monomial {
            factor: 5.,
            variable: 'x',
            power: 1,
        });
        let right_monomial = Expression::Monomial(Monomial {
            factor: 3.,
            variable: 'y',
            power: 2,
        });
        let product = Expression::BinaryOperation(BinaryOperation {
            operation: BinaryOperationType::Multiplication,
            left_value: Box::new(left_monomial.clone()),
            right_value: Box::new(right_monomial.clone()),
        });

        assert_eq!(
            derive(&product, 'x'),
            Expression::Monomial(Monomial {
                factor: 15.,
                variable: 'y',
                power: 2
            })
        );
        assert_eq!(
            derive(&product, 'y'),
            Expression::BinaryOperation(BinaryOperation {
                operation: BinaryOperationType::Multiplication,
                left_value: Box::new(Expression::Monomial(Monomial {
                    factor: 6.,
                    variable: 'y',
                    power: 1
                })),
                right_value: Box::new(Expression::Monomial(Monomial {
                    factor: 5.,
                    variable: 'x',
                    power: 1
                }))
            })
        );
        assert_eq!(derive(&product, 'z'), Expression::Constant(0.));
    }

    #[test]
    fn test_simplify_expression() {
        let expr = Expression::Constant(5.);
        assert_eq!(simplify_expression(expr.clone()), expr);

        let expr = Expression::Monomial(Monomial {
            factor: 5.,
            variable: 'x',
            power: 2,
        });
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
        let expr = Expression::BinaryOperation(BinaryOperation {
            operation: BinaryOperationType::Addition,
            left_value: Box::new(value.clone()),
            right_value: Box::new(Expression::Constant(0.)),
        });
        assert_eq!(simplify_expression(expr), value);

        let expr = Expression::BinaryOperation(BinaryOperation {
            operation: BinaryOperationType::Multiplication,
            left_value: Box::new(value.clone()),
            right_value: Box::new(Expression::Constant(1.)),
        });
        assert_eq!(simplify_expression(expr), value);

        let expr = Expression::BinaryOperation(BinaryOperation {
            operation: BinaryOperationType::Multiplication,
            left_value: Box::new(value.clone()),
            right_value: Box::new(Expression::Constant(0.)),
        });
        assert_eq!(simplify_expression(expr), Expression::Constant(0.));
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
            simplify_expression(expr),
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
            simplify_expression(Expression::BinaryOperation(BinaryOperation {
                operation: BinaryOperationType::Multiplication,
                left_value: Box::new(Expression::Constant(5.)),
                right_value: Box::new(Expression::Monomial(Monomial {
                    factor: 3.,
                    variable: 'x',
                    power: 2
                }))
            })),
            Expression::Monomial(Monomial {
                factor: 15.,
                variable: 'x',
                power: 2
            })
        );

        assert_eq!(
            simplify_expression(Expression::BinaryOperation(BinaryOperation {
                operation: BinaryOperationType::Multiplication,
                left_value: Box::new(Expression::Monomial(Monomial {
                    factor: 3.,
                    variable: 'x',
                    power: 2
                })),
                right_value: Box::new(Expression::Constant(5.)),
            })),
            Expression::Monomial(Monomial {
                factor: 15.,
                variable: 'x',
                power: 2
            })
        );
    }
}
