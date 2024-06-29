use std::collections::HashSet;

use crate::expression::{Expression, Variable};

pub struct Surface {
    pub surface_variables: HashSet<Variable>,
    pub parametric_variables: HashSet<Variable>,
}

impl Surface {
    pub fn from_embedding(
        surface_variables: HashSet<Variable>,
        embedding: Vec<Expression>,
    ) -> Surface {
        let all_variables: HashSet<Variable> =
            embedding.iter().flat_map(|expr| expr.variables()).collect();

        Surface {
            surface_variables: surface_variables.clone(),
            parametric_variables: all_variables
                .difference(&surface_variables)
                .copied()
                .collect(),
        }
    }
}
