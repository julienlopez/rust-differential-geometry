use std::collections::HashSet;

use rust_differential_geometry::expression::{
    BinaryOperation, BinaryOperationType, Expression, Function,
};
use rust_differential_geometry::surface::Surface;

#[test]
fn it_adds_two() {
    let torus = Surface::from_embedding(
        HashSet::from(['u', 'v']),
        vec![
            Expression::Operation(BinaryOperation {
                operation: BinaryOperationType::Multiplication,
                left_value: Box::new(Expression::Function {
                    function: Function::Cosine,
                    expression: Box::new(Expression::Monomial {
                        factor: 1.,
                        variable: 'u',
                        power: 1,
                    }),
                }),
                right_value: Box::new(Expression::Operation(BinaryOperation {
                    operation: BinaryOperationType::Addition,
                    left_value: Box::new(Expression::Monomial {
                        factor: 1.,
                        variable: 'R',
                        power: 1,
                    }),
                    right_value: Box::new(Expression::Operation(BinaryOperation {
                        operation: BinaryOperationType::Multiplication,
                        left_value: Box::new(Expression::Monomial {
                            factor: 1.,
                            variable: 'r',
                            power: 1,
                        }),
                        right_value: Box::new(Expression::Function {
                            function: Function::Cosine,
                            expression: Box::new(Expression::Monomial {
                                factor: 1.,
                                variable: 'v',
                                power: 1,
                            }),
                        }),
                    })),
                })),
            }),
            Expression::Operation(BinaryOperation {
                operation: BinaryOperationType::Multiplication,
                left_value: Box::new(Expression::Function {
                    function: Function::Cosine,
                    expression: Box::new(Expression::Monomial {
                        factor: 1.,
                        variable: 'u',
                        power: 1,
                    }),
                }),
                right_value: Box::new(Expression::Operation(BinaryOperation {
                    operation: BinaryOperationType::Addition,
                    left_value: Box::new(Expression::Monomial {
                        factor: 1.,
                        variable: 'R',
                        power: 1,
                    }),
                    right_value: Box::new(Expression::Operation(BinaryOperation {
                        operation: BinaryOperationType::Multiplication,
                        left_value: Box::new(Expression::Monomial {
                            factor: 1.,
                            variable: 'r',
                            power: 1,
                        }),
                        right_value: Box::new(Expression::Function {
                            function: Function::Sine,
                            expression: Box::new(Expression::Monomial {
                                factor: 1.,
                                variable: 'v',
                                power: 1,
                            }),
                        }),
                    })),
                })),
            }),
            Expression::Operation(BinaryOperation {
                operation: BinaryOperationType::Multiplication,
                left_value: Box::new(Expression::Monomial {
                    factor: 1.,
                    variable: 'r',
                    power: 1,
                }),
                right_value: Box::new(Expression::Function {
                    function: Function::Sine,
                    expression: Box::new(Expression::Monomial {
                        factor: 1.,
                        variable: 'v',
                        power: 1,
                    }),
                }),
            }),
        ],
    );
    assert_eq!(torus.surface_variables, HashSet::from(['u', 'v']));
    assert_eq!(torus.parametric_variables, HashSet::from(['r', 'R']));
}
