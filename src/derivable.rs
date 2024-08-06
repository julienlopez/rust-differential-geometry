use crate::expression::{
    BinaryOperation, BinaryOperationType, Expression, Function, Monomial, Variable,
};
use crate::simplifiable::*;

pub trait Derivable {
    fn derive(&self, derivation_variable: Variable) -> Self;
}

impl Derivable for Expression {
    fn derive(&self, derivation_variable: Variable) -> Expression {
        match self {
            Expression::Constant(_) | Expression::NamedConstant(_) => Expression::Constant(0.),
            Expression::Monomial(m) => {
                derive_monomial(derivation_variable, m).simplify_expression()
            }
            Expression::BinaryOperation(operation) => {
                derive_operation(derivation_variable, operation).simplify_expression()
            }
            Expression::Function {
                function,
                expression,
            } => derive_function(*function, expression, derivation_variable),
            _ => unimplemented!(),
        }
    }
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
                left_value: Box::new(operation.left_value.derive(derivation_variable)),
                right_value: Box::new(operation.right_value.derive(derivation_variable)),
            })
        }
        BinaryOperationType::Multiplication => Expression::BinaryOperation(BinaryOperation {
            operation: BinaryOperationType::Addition,
            left_value: Box::new(Expression::BinaryOperation(BinaryOperation {
                operation: BinaryOperationType::Multiplication,
                left_value: Box::new(operation.left_value.derive(derivation_variable)),
                right_value: Box::new(*operation.right_value.clone()),
            })),
            right_value: Box::new(Expression::BinaryOperation(BinaryOperation {
                operation: BinaryOperationType::Multiplication,
                left_value: Box::new(operation.right_value.derive(derivation_variable)),
                right_value: Box::new(*operation.left_value.clone()),
            })),
        }),
        _ => unimplemented!(),
    }
}

fn derive_function(
    function: Function,
    expression: &Expression,
    derivation_variable: Variable,
) -> Expression {
    if !expression.variables().contains(&derivation_variable) {
        return Expression::Constant(0.);
    }
    match function {
        Function::Cosine => {
            unimplemented!()
        }
        Function::Sine => Expression::Function {
            function: Function::Cosine,
            expression: Box::new(expression.clone()),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::derivable::*;

    #[test]
    fn derive_constant() {
        assert_eq!(
            Expression::Constant(5.).derive('x'),
            Expression::Constant(0.)
        );

        assert_eq!(
            Expression::NamedConstant("pi").derive('x'),
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
        assert_eq!(x_monomial.derive('y'), Expression::Constant(0.));
        assert_eq!(x_monomial.derive('x'), Expression::Constant(5.));

        let x_squared_monomial = Expression::Monomial(Monomial {
            factor: 3.,
            variable: 'x',
            power: 2,
        });
        assert_eq!(x_squared_monomial.derive('y'), Expression::Constant(0.));
        assert_eq!(
            x_squared_monomial.derive('x'),
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
        assert_eq!(x_3_monomial.derive('y'), Expression::Constant(0.));
        assert_eq!(
            x_3_monomial.derive('x'),
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

        assert_eq!(sum.derive('y'), Expression::Constant(0.));
        assert_eq!(
            sum.derive('x'),
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
            product.derive('x'),
            Expression::Monomial(Monomial {
                factor: 15.,
                variable: 'y',
                power: 2
            })
        );
        assert_eq!(
            product.derive('y'),
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
        assert_eq!(product.derive('z'), Expression::Constant(0.));
    }

    #[test]
    fn test_derive_simple_functions() {
        let expr = Expression::Function {
            function: Function::Sine,
            expression: Box::new(Expression::Monomial(Monomial {
                factor: 1.0,
                variable: 'x',
                power: 1,
            })),
        };
        assert_eq!(expr.derive('y'), Expression::Constant(0.));
        assert_eq!(
            expr.derive('x'),
            Expression::Function {
                function: Function::Cosine,
                expression: Box::new(Expression::Monomial(Monomial {
                    factor: 1.0,
                    variable: 'x',
                    power: 1,
                }))
            }
        );
    }
}
