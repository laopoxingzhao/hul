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

#[test]
fn truthiness_rules() {
    // 0, nil, false, 空串 为假
    let interp = run_program(
        r#"
        let r0 = 0;   let r1 = 1;
        let s0 = "";  let s1 = "hi";
        let b0 = false; let b1 = true;
        let n  = nil;
        if (r0) { r0 = 999; }
        if (r1) { r1 = 1; }
        if (s0) { s0 = "x"; }
        if (s1) { s1 = "hi"; }
        if (b0) { b0 = true; }
        if (b1) { b1 = true; }
        if (n)  { n  = 1; }
    "#,
    )
    .unwrap();
    let env = interp.env.borrow();
    // 0 → 不进入 if，保持 0
    assert_eq!(*env.get("r0").unwrap().borrow(), Value::Number(0.0));
    // 1 → 进入 if
    assert_eq!(*env.get("r1").unwrap().borrow(), Value::Number(1.0));
    // "" → 不进入 if
    assert_eq!(*env.get("s0").unwrap().borrow(), Value::String("".to_string()));
    // "hi" → 进入 if
    assert_eq!(*env.get("s1").unwrap().borrow(), Value::String("hi".to_string()));
    // false → 不进入 if
    assert_eq!(*env.get("b0").unwrap().borrow(), Value::Boolean(false));
    // true → 进入 if
    assert_eq!(*env.get("b1").unwrap().borrow(), Value::Boolean(true));
    // nil → 不进入 if
    assert_eq!(*env.get("n").unwrap().borrow(), Value::Nil);
}

#[test]
fn string_concatenation() {
    let interp = run_program(
        r#"
        let s1 = "hello" + " " + "world";
        let s2 = "value=" + 42;
        let s3 = 100 + "%";
    "#,
    )
    .unwrap();
    let env = interp.env.borrow();
    assert_eq!(
        *env.get("s1").unwrap().borrow(),
        Value::String("hello world".to_string())
    );
    assert_eq!(
        *env.get("s2").unwrap().borrow(),
        Value::String("value=42".to_string())
    );
    assert_eq!(
        *env.get("s3").unwrap().borrow(),
        Value::String("100%".to_string())
    );
}

#[test]
fn for_loop_sum() {
    let interp = run_program(
        r#"
        let sum = 0;
        for (let i = 1; i <= 10; i = i + 1) {
            sum = sum + i;
        }
    "#,
    )
    .unwrap();
    let env = interp.env.borrow();
    assert_eq!(*env.get("sum").unwrap().borrow(), Value::Number(55.0));
}

#[test]
fn for_loop_scope() {
    // for 循环中声明的变量应存活于循环内
    let interp = run_program(
        r#"
        let result = 0;
        for (let i = 0; i < 5; i = i + 1) {
            let local = i * 2;
            result = result + local;
        }
    "#,
    )
    .unwrap();
    let env = interp.env.borrow();
    // 0 + 2 + 4 + 6 + 8 = 20
    assert_eq!(*env.get("result").unwrap().borrow(), Value::Number(20.0));
}

#[test]
fn for_loop_no_initializer() {
    let interp = run_program(
        r#"
        let i = 0;
        let count = 0;
        for (; i < 3; i = i + 1) {
            count = count + 1;
        }
    "#,
    )
    .unwrap();
    let env = interp.env.borrow();
    assert_eq!(*env.get("count").unwrap().borrow(), Value::Number(3.0));
}

#[test]
fn break_in_while() {
    let interp = run_program(
        r#"
        let sum = 0;
        let i = 0;
        while (true) {
            if (i >= 5) { break; }
            sum = sum + i;
            i = i + 1;
        }
    "#,
    )
    .unwrap();
    let env = interp.env.borrow();
    // 0+1+2+3+4 = 10
    assert_eq!(*env.get("sum").unwrap().borrow(), Value::Number(10.0));
}

#[test]
fn continue_in_while() {
    let interp = run_program(
        r#"
        let sum = 0;
        let i = 0;
        while (i < 5) {
            i = i + 1;
            if (i == 3) { continue; }
            sum = sum + i;
        }
    "#,
    )
    .unwrap();
    let env = interp.env.borrow();
    // 1+2+4+5 = 12 (跳过 3)
    assert_eq!(*env.get("sum").unwrap().borrow(), Value::Number(12.0));
}

#[test]
fn break_in_for() {
    let interp = run_program(
        r#"
        let sum = 0;
        for (let i = 0; i < 100; i = i + 1) {
            if (i == 5) { break; }
            sum = sum + i;
        }
    "#,
    )
    .unwrap();
    let env = interp.env.borrow();
    // 0+1+2+3+4 = 10
    assert_eq!(*env.get("sum").unwrap().borrow(), Value::Number(10.0));
}

#[test]
fn continue_in_for() {
    let interp = run_program(
        r#"
        let sum = 0;
        for (let i = 0; i < 5; i = i + 1) {
            if (i == 2) { continue; }
            sum = sum + i;
        }
    "#,
    )
    .unwrap();
    let env = interp.env.borrow();
    // 0+1+3+4 = 8 (跳过 2)
    assert_eq!(*env.get("sum").unwrap().borrow(), Value::Number(8.0));
}

#[test]
fn break_nested_loops() {
    // break 只跳出最内层循环
    let interp = run_program(
        r#"
        let count = 0;
        for (let i = 0; i < 3; i = i + 1) {
            for (let j = 0; j < 3; j = j + 1) {
                if (j == 1) { break; }
                count = count + 1;
            }
        }
    "#,
    )
    .unwrap();
    let env = interp.env.borrow();
    // 外层3次，内层每次只执行 j=0 时 +1
    assert_eq!(*env.get("count").unwrap().borrow(), Value::Number(3.0));
}
