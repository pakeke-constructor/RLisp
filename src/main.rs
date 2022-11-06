
// https://crates.io/crates/logos

mod rlisp;





 
fn main() {
    let x = "(print (+ 1 2))";

    let toks = rlisp::tokenify(x);

    let ast = rlisp::parse(toks);
    
    let mut scope = rlisp::make_scope();

    rlisp::execute(ast, &mut scope);

    println!("\n");
}


