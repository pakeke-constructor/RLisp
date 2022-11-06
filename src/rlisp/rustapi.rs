
/**
 * The API to use when defining rlisp functions.
 * 
 */



use crate::rlisp::types::*;



fn check_stack(f: &FunctionEnv, index: usize) -> &Value {
    if index > f.stack.len() {
        panic!("Function argument index exceeds stack size");
        // TODO: make this nicer
    }
    return f.stack.get(index).unwrap();
}



pub fn get_number(f: &FunctionEnv, index: usize) -> Number {
    let v = check_stack(f,index);
    if let Value::Number(x) = v {
        return *x;
    }
    panic!("Not a number"); // TODO: make this argument format nicer
}



pub fn get_string(f: &FunctionEnv, index: usize) -> &String {
    let v = check_stack(f,index);
    if let Value::String(x) = v {
        return x
    }
    panic!("Not a string"); // TODO make this argument format nicer
}



pub fn push_return(f: &mut FunctionEnv, v: Value) {
    f.retval = v;
}



