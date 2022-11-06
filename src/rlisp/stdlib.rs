
use std::cell::RefCell;
use std::rc::Rc;

use crate::rlisp::types::{FunctionEnv, Value, Scope, ForeignFunction};
use crate::rlisp::rustapi::{push_return, get_number, get_string};



fn plus(f: &mut FunctionEnv) {
    let x = get_number(f, 1);
    let y = get_number(f, 2);
    let ret = Value::Number(x + y);
    push_return(f, ret);
}


fn print(f: &mut FunctionEnv) {
    let s = get_string(f, 1);
    println!("{}", s);
}


fn minus(f: &mut FunctionEnv) {
    let x = get_number(f, 1);
    let y = get_number(f, 2);
    let ret = Value::Number(x - y);
    push_return(f, ret);
}



fn times(f: &mut FunctionEnv) {
    let x = get_number(f, 1);
    let y = get_number(f, 2);
    let ret = Value::Number(x * y);
    push_return(f, ret);
}



fn divide(f: &mut FunctionEnv) {
    let x = get_number(f, 1);
    let y = get_number(f, 2);
    let ret = Value::Number(x / y);
    push_return(f, ret);
}





fn prepare_value(v: Value) -> Rc<RefCell<Value>> {
    let refcell = RefCell::new(v);
    let rc = Rc::new(refcell);
    return rc
}



fn put_function(scope: &mut Scope, string: &str, func: ForeignFunction) {
    let v = Value::ForeignFunction(func);
    let rc = prepare_value(v);
    scope.local.insert(string.to_string(), rc);
}


pub fn export_default_lib(scope: &mut Scope) {
    put_function(scope, "+", plus);
    put_function(scope, "-", minus);
    put_function(scope, "*", times);
    put_function(scope, "/", divide);
    put_function(scope, "print", print)
}


