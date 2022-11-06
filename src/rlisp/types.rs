
use std::borrow::Borrow;
use std::cell::{RefCell};
use std::rc::Rc;


pub type Number = f64;

pub type String = std::string::String;

pub type Object = std::collections::HashMap<String, Rc<Value>>;

pub type Function = u32; // TODO


pub type ForeignFunction = fn (&mut FunctionEnv) -> (); 
pub type ForeignObject = u32; // TODO


const NOT_ALLOWED_CHAR: char = '`';


pub enum Literal {
    True,
    False,
    Nil
}


pub enum Type {
    Number, // f64
    String,
    Object, // like lua table
    Literal, // true false nil
        
    ForeignFunction, // foreign rust function
    ForeignObject // foreign rust object
}


pub enum Value {
    Nil,
    False,
    True,
    Number(Number),
    Object(RefCell<Object>),
    String(String),
    Function(Function),
    ForeignFunction(ForeignFunction),
    ForeignObject(ForeignObject)
}

pub type RLispValue = Rc<Value>;



pub struct Scope {
    pub local: Object, // local scope
    pub parent: Option<Object> // parent scope
}



pub struct FunctionEnv {
    pub scope: Scope,
    pub stack: Vec<Value>,
    pub retval: Value // TODO: Allow multiple returns
}



impl Value {
    fn is_func(&self) -> bool {
        // returns true if the value is a foreign function, or regular function.
        return match self {
            Value::Function(_) => true,
            Value::ForeignFunction(_) => true,
            _ => false
        }
    }

    fn get_number(&self) -> Number {
        if let Value::Number(x) = self {
            return *x;
        } else {
            std::panic!("Attempted to get number from non-number")
        }
    }

    fn get_object(&self) -> &Object {
        if let Value::Object(x) = self {
            return &x.borrow();
        } else {
            std::panic!("Attempted to get object from non-object")
        }
    }

    fn get_string(&self) -> &String {
        if let Value::String(x) = self {
            return x;
        } else {
            std::panic!("Attempted to get string from non-string")
        }
    }

    fn put(&mut self, key: String, value: Value) {
        if let Value::Object(x) = self {
            let obj = x.borrow_mut();
            let rval = Rc::new(value);
            obj.insert(key, rval);
        } else {
            std::panic!("Attempted to use put on a non-object");
        }
    }
}



