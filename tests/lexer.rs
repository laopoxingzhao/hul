use hul::lexer::{Lexer, Token, TokenType};

fn tokenize(source: &str) -> Vec<Token> {
    Lexer::new(source).collect_tokens().unwrap()
}

// ==================== 1. 数字字面量测试 ====================

#[test]
fn test_integer_literal() {
    let result = tokenize("42");
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].ty, TokenType::Number(42.0));
}

#[test]
fn test_negative_integer() {
    let result = tokenize("-5");
    assert_eq!(result.len(), 2); // - 和 5
    assert_eq!(result[0].ty, TokenType::Minus);
    assert_eq!(result[1].ty, TokenType::Number(5.0));
}

#[test]
fn test_float_literal() {
    let result = tokenize("3.14");
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].ty, TokenType::Number(3.14));
}

#[test]
fn test_float_with_leading_zero() {
    let result = tokenize("0.5");
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].ty, TokenType::Number(0.5));
}

#[test]
fn test_large_number() {
    let result = tokenize("1234567890");
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].ty, TokenType::Number(1234567890.0));
}

// ==================== 2. 字符串字面量测试 ====================

#[test]
fn test_string_literal() {
    let result = tokenize("\"hello\"");
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].ty, TokenType::String("hello".to_string()));
}

#[test]
fn test_empty_string() {
    let result = tokenize("\"\"");
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].ty, TokenType::String("".to_string()));
}

#[test]
fn test_string_with_spaces() {
    let result = tokenize("\"hello world\"");
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].ty, TokenType::String("hello world".to_string()));
}

#[test]
fn test_string_with_numbers() {
    let result = tokenize("\"test123\"");
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].ty, TokenType::String("test123".to_string()));
}

// ==================== 2.1 字符串转义测试 ====================

#[test]
fn test_escape_newline() {
    let result = tokenize(r#""hello\nworld""#);
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].ty, TokenType::String("hello\nworld".to_string()));
}

#[test]
fn test_escape_tab() {
    let result = tokenize(r#""hello\tworld""#);
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].ty, TokenType::String("hello\tworld".to_string()));
}

#[test]
fn test_escape_backslash() {
    let result = tokenize(r#""hello\\world""#);
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].ty, TokenType::String("hello\\world".to_string()));
}

#[test]
fn test_escape_quote() {
    let result = tokenize(r#""hello\"world""#);
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].ty, TokenType::String("hello\"world".to_string()));
}

#[test]
fn test_escape_carriage_return() {
    let result = tokenize(r#""hello\rworld""#);
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].ty, TokenType::String("hello\rworld".to_string()));
}

#[test]
fn test_multiple_escapes() {
    let result = tokenize(r#""line1\nline2\ttab""#);
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].ty, TokenType::String("line1\nline2\ttab".to_string()));
}

// ==================== 3. 布尔值和 nil 测试 ====================

#[test]
fn test_true_keyword() {
    let result = tokenize("true");
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].ty, TokenType::True);
}

#[test]
fn test_false_keyword() {
    let result = tokenize("false");
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].ty, TokenType::False);
}

#[test]
fn test_nil_keyword() {
    let result = tokenize("nil");
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].ty, TokenType::Nil);
}

// ==================== 4. 关键字测试 ====================

#[test]
fn test_let_keyword() {
    let result = tokenize("let");
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].ty, TokenType::Let);
}

#[test]
fn test_if_keyword() {
    let result = tokenize("if");
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].ty, TokenType::If);
}

#[test]
fn test_else_keyword() {
    let result = tokenize("else");
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].ty, TokenType::Else);
}

#[test]
fn test_while_keyword() {
    let result = tokenize("while");
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].ty, TokenType::While);
}

#[test]
fn test_for_keyword() {
    let result = tokenize("for");
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].ty, TokenType::For);
}

#[test]
fn test_break_keyword() {
    let result = tokenize("break");
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].ty, TokenType::Break);
}

#[test]
fn test_continue_keyword() {
    let result = tokenize("continue");
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].ty, TokenType::Continue);
}

#[test]
fn test_print_keyword() {
    let result = tokenize("print");
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].ty, TokenType::Print);
}

#[test]
fn test_and_keyword() {
    let result = tokenize("and");
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].ty, TokenType::And);
}

#[test]
fn test_or_keyword() {
    let result = tokenize("or");
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].ty, TokenType::Or);
}

#[test]
fn test_not_keyword() {
    let result = tokenize("not");
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].ty, TokenType::Not);
}

// ==================== 5. 标识符测试 ====================

#[test]
fn test_simple_identifier() {
    let result = tokenize("x");
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].ty, TokenType::Identifier("x".to_string()));
}

#[test]
fn test_identifier_with_underscore() {
    let result = tokenize("my_var");
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].ty, TokenType::Identifier("my_var".to_string()));
}

#[test]
fn test_identifier_with_numbers() {
    let result = tokenize("var123");
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].ty, TokenType::Identifier("var123".to_string()));
}

#[test]
fn test_identifier_starting_with_underscore() {
    let result = tokenize("_private");
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].ty, TokenType::Identifier("_private".to_string()));
}

#[test]
fn test_long_identifier() {
    let result = tokenize("myVeryLongVariableName123");
    assert_eq!(result.len(), 1);
    assert_eq!(
        result[0].ty,
        TokenType::Identifier("myVeryLongVariableName123".to_string())
    );
}

// ==================== 6. 运算符测试 ====================

#[test]
fn test_plus_operator() {
    let result = tokenize("+");
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].ty, TokenType::Plus);
}

#[test]
fn test_minus_operator() {
    let result = tokenize("-");
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].ty, TokenType::Minus);
}

#[test]
fn test_star_operator() {
    let result = tokenize("*");
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].ty, TokenType::Star);
}

#[test]
fn test_slash_operator() {
    let result = tokenize("/ ");
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].ty, TokenType::Slash);
}

#[test]
fn test_percent_operator() {
    let result = tokenize("%");
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].ty, TokenType::Percent);
}

#[test]
fn test_equal_equal_operator() {
    let result = tokenize("==");
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].ty, TokenType::EqualEqual);
}

#[test]
fn test_bang_equal_operator() {
    let result = tokenize("!=");
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].ty, TokenType::BangEqual);
}

#[test]
fn test_less_operator() {
    let result = tokenize("<");
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].ty, TokenType::Less);
}

#[test]
fn test_less_equal_operator() {
    let result = tokenize("<=");
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].ty, TokenType::LessEqual);
}

#[test]
fn test_greater_operator() {
    let result = tokenize(">");
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].ty, TokenType::Greater);
}

#[test]
fn test_greater_equal_operator() {
    let result = tokenize(">=");
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].ty, TokenType::GreaterEqual);
}

#[test]
fn test_assign_operator() {
    let result = tokenize("=");
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].ty, TokenType::Assign);
}

// ==================== 7. 分隔符测试 ====================

#[test]
fn test_left_paren() {
    let result = tokenize("(");
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].ty, TokenType::LParen);
}

#[test]
fn test_right_paren() {
    let result = tokenize(")");
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].ty, TokenType::RParen);
}

#[test]
fn test_left_brace() {
    let result = tokenize("{");
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].ty, TokenType::LBrace);
}

#[test]
fn test_right_brace() {
    let result = tokenize("}");
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].ty, TokenType::RBrace);
}

#[test]
fn test_semicolon() {
    let result = tokenize(";");
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].ty, TokenType::Semicolon);
}

#[test]
fn test_all_delimiters() {
    let result = tokenize("(){};");
    assert_eq!(result.len(), 5);
    assert_eq!(result[0].ty, TokenType::LParen);
    assert_eq!(result[1].ty, TokenType::RParen);
    assert_eq!(result[2].ty, TokenType::LBrace);
    assert_eq!(result[3].ty, TokenType::RBrace);
    assert_eq!(result[4].ty, TokenType::Semicolon);
}

// ==================== 8. 复合表达式测试 ====================

#[test]
fn test_variable_declaration() {
    let result = tokenize("let x = 42;");
    assert_eq!(result.len(), 5);
    assert_eq!(result[0].ty, TokenType::Let);
    assert_eq!(result[1].ty, TokenType::Identifier("x".to_string()));
    assert_eq!(result[2].ty, TokenType::Assign);
    assert_eq!(result[3].ty, TokenType::Number(42.0));
    assert_eq!(result[4].ty, TokenType::Semicolon);
}

#[test]
fn test_if_statement() {
    let result = tokenize("if (x) { y; }");
    assert_eq!(result.len(), 8);
    assert_eq!(result[0].ty, TokenType::If);
    assert_eq!(result[1].ty, TokenType::LParen);
    assert_eq!(result[2].ty, TokenType::Identifier("x".to_string()));
    assert_eq!(result[3].ty, TokenType::RParen);
    assert_eq!(result[4].ty, TokenType::LBrace);
    assert_eq!(result[5].ty, TokenType::Identifier("y".to_string()));
    assert_eq!(result[6].ty, TokenType::Semicolon);
    assert_eq!(result[7].ty, TokenType::RBrace);
}

#[test]
fn test_while_loop() {
    let result = tokenize("while (x < 10) { x = x + 1; }");
    assert_eq!(result.len(), 14);
    assert_eq!(result[0].ty, TokenType::While);
    assert_eq!(result[1].ty, TokenType::LParen);
}

#[test]
fn test_arithmetic_expression() {
    let result = tokenize("a + b * c - e");
    assert_eq!(result.len(), 7);
}

#[test]
fn test_logical_expression() {
    let result = tokenize("a and b or not c");
    assert_eq!(result.len(), 6);
    assert_eq!(result[1].ty, TokenType::And);
    assert_eq!(result[3].ty, TokenType::Or);
    assert_eq!(result[4].ty, TokenType::Not);
}

#[test]
fn test_comparison_chain() {
    let result = tokenize("a == b and c != d");
    assert_eq!(result.len(), 7);
}

#[test]
fn test_print_statement() {
    let result = tokenize("print \"hello\";");
    assert_eq!(result.len(), 3);
    assert_eq!(result[0].ty, TokenType::Print);
    assert_eq!(result[1].ty, TokenType::String("hello".to_string()));
    assert_eq!(result[2].ty, TokenType::Semicolon);
}

// ==================== 9. 注释跳过测试 ====================

#[test]
fn test_single_line_comment() {
    let result = tokenize("42 // this is a comment");
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].ty, TokenType::Number(42.0));
}

#[test]
fn test_comment_with_newline() {
    let result = tokenize("42 // comment\n56");
    assert_eq!(result.len(), 2);
    assert_eq!(result[0].ty, TokenType::Number(42.0));
    assert_eq!(result[1].ty, TokenType::Number(56.0));
}

#[test]
fn test_comment_at_start() {
    let result = tokenize("// comment\nlet x = 1;");
    assert_eq!(result.len(), 5);
    assert_eq!(result[0].ty, TokenType::Let);
}

#[test]
fn test_multiple_comments() {
    let result = tokenize("// comment 1\n42 // comment 2\n// comment 3\n56");
    assert_eq!(result.len(), 2);
    assert_eq!(result[0].ty, TokenType::Number(42.0));
    assert_eq!(result[1].ty, TokenType::Number(56.0));
}

#[test]
fn test_comment_in_middle() {
    let result = tokenize("let // comment\nx = 1;");
    assert_eq!(result.len(), 5);
    assert_eq!(result[0].ty, TokenType::Let);
    assert_eq!(result[1].ty, TokenType::Identifier("x".to_string()));
}

// ==================== 9.1 多行注释测试 ====================

#[test]
fn test_block_comment_basic() {
    let result = tokenize("42 /* comment */ 56");
    assert_eq!(result.len(), 2);
    assert_eq!(result[0].ty, TokenType::Number(42.0));
    assert_eq!(result[1].ty, TokenType::Number(56.0));
}

#[test]
fn test_block_comment_multiline() {
    let source = "42 /* line 1\nline 2\nline 3 */ 56";
    let result = tokenize(source);
    assert_eq!(result.len(), 2);
    assert_eq!(result[0].ty, TokenType::Number(42.0));
    assert_eq!(result[1].ty, TokenType::Number(56.0));
}

#[test]
fn test_block_comment_inline() {
    let result = tokenize("let x = 2; /* inline comment */ let y = 3;");
    assert_eq!(result.len(), 10);
    assert_eq!(result[0].ty, TokenType::Let);
    assert_eq!(result[5].ty, TokenType::Let);
}

#[test]
fn test_block_comment_at_start() {
    let result = tokenize("/* comment */ let x = 1;");
    assert_eq!(result.len(), 5);
    assert_eq!(result[0].ty, TokenType::Let);
}

#[test]
fn test_block_comment_empty() {
    let result = tokenize("42 /**/ 56");
    assert_eq!(result.len(), 2);
    assert_eq!(result[0].ty, TokenType::Number(42.0));
    assert_eq!(result[1].ty, TokenType::Number(56.0));
}

#[test]
fn test_block_comment_with_special_chars() {
    let result = tokenize("42 /* @#$%^&*() */ 56");
    assert_eq!(result.len(), 2);
    assert_eq!(result[0].ty, TokenType::Number(42.0));
    assert_eq!(result[1].ty, TokenType::Number(56.0));
}

#[test]
fn test_mixed_comments() {
    let source = "// single line\n42 /* block\ncomment */ 56 // another";
    let result = tokenize(source);
    assert_eq!(result.len(), 2);
    assert_eq!(result[0].ty, TokenType::Number(42.0));
    assert_eq!(result[1].ty, TokenType::Number(56.0));
}

#[test]
fn test_block_comment_in_code() {
    let result = tokenize("let /* comment */ x = 1;");
    assert_eq!(result.len(), 5);
    assert_eq!(result[0].ty, TokenType::Let);
    assert_eq!(result[1].ty, TokenType::Identifier("x".to_string()));
}

// ==================== 10. 位置信息测试 ====================

#[test]
fn test_position_single_token() {
    let result = tokenize("let");
    assert_eq!(result[0].line, 1);
    assert_eq!(result[0].col, 1);
}

#[test]
fn test_position_multiple_tokens() {
    let result = tokenize("let x = 1");
    assert_eq!(result[0].line, 1);
    assert_eq!(result[0].col, 1);
    assert_eq!(result[1].line, 1);
    assert_eq!(result[1].col, 5);
    assert_eq!(result[2].line, 1);
    assert_eq!(result[2].col, 7);
    assert_eq!(result[3].line, 1);
    assert_eq!(result[3].col, 9);
}

#[test]
fn test_position_with_spaces() {
    let result = tokenize("  let    x  =  1  ");
    assert_eq!(result[0].line, 1);
    assert_eq!(result[0].col, 3);
    assert_eq!(result[1].line, 1);
    assert_eq!(result[1].col, 10);
    assert_eq!(result[2].line, 1);
    assert_eq!(result[2].col, 13);
    assert_eq!(result[3].line, 1);
    assert_eq!(result[3].col, 16);
}

#[test]
fn test_position_newline() {
    let result = tokenize("let x = 1\nlet y = 2");
    assert_eq!(result[0].line, 1);
    assert_eq!(result[0].col, 1);
    assert_eq!(result[1].line, 1);
    assert_eq!(result[1].col, 5);
    assert_eq!(result[4].line, 2);
    assert_eq!(result[4].col, 1);
    assert_eq!(result[5].line, 2);
    assert_eq!(result[5].col, 5);
}

#[test]
fn test_position_multiple_newlines() {
    let result = tokenize("let x = 1\n\n\nlet y = 2");
    assert_eq!(result[0].line, 1);
    assert_eq!(result[4].line, 4);
}

// ==================== 边界情况测试 ====================

#[test]
fn test_empty_input() {
    let result = tokenize("");
    assert!(result.is_empty());
}

#[test]
fn test_whitespace_only() {
    let result = tokenize("   \n\t  ");
    assert!(result.is_empty());
}

#[test]
fn test_leading_whitespace() {
    let result = tokenize("   42");
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].ty, TokenType::Number(42.0));
}

#[test]
fn test_trailing_whitespace() {
    let result = tokenize("42   ");
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].ty, TokenType::Number(42.0));
}

#[test]
fn test_complex_expression() {
    let result = tokenize("let result = (a + b) * c - 2;");
    assert!(result.len() > 0);
    assert_eq!(result[0].ty, TokenType::Let);
    assert_eq!(result[3].ty, TokenType::LParen);
    assert_eq!(result[7].ty, TokenType::RParen);
}

#[test]
fn test_nested_parens() {
    let result = tokenize("((a))");
    assert_eq!(result.len(), 5);
}

#[test]
fn test_bool_and_nil_in_expression() {
    let result = tokenize("true and false or nil");
    assert!(result.len() >= 3);
    assert_eq!(result[0].ty, TokenType::True);
    assert_eq!(result[2].ty, TokenType::False);
    assert_eq!(result[4].ty, TokenType::Nil);
}
