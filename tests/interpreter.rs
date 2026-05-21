use hul::{Interpreter, Parser, Value};

fn run_program(source: &str) -> Result<Interpreter, String> {
    let mut parser = Parser::new(source)?;
    let stmts = parser.parse_program()?;
    let mut interp = Interpreter::new();
    interp.interpret(&stmts)?;
    Ok(interp)
}

#[test]
fn evaluate_chain_assignment() {
    let interp = run_program("let a = 0; let b = 0; a = b = 3;").unwrap();
    let a_val = interp.env.borrow().get("a").unwrap();
    let b_val = interp.env.borrow().get("b").unwrap();
    assert_eq!(*a_val.borrow(), Value::Number(3.0));
    assert_eq!(*b_val.borrow(), Value::Number(3.0));
}

#[test]
fn function_definition_and_call() {
    let interp = run_program(
        "fn add(a, b) { return a + b; } let x = add(2, 3);",
    )
    .unwrap();
    let x_val = interp.env.borrow().get("x").unwrap();
    assert_eq!(*x_val.borrow(), Value::Number(5.0));
}
