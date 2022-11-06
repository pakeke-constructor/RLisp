

use crate::rlisp::ast::*;
use crate::rlisp::types::*;

use std::collections::HashMap;




pub fn execute_expression(scope: Scope, expression: AST) {
    let f: FunctionEnv = FunctionEnv {
        scope: Scope {
            local: HashMap::new(),
            parent: Some(scope.local)
        },
        stack: Vec::new(), 
        retval: Value::Nil
    };

    
}




