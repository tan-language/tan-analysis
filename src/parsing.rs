use tan::{error::Error, expr::Expr, lexer::Lexer, parser::Parser};

// #todo #temp This is a temp helper method.
pub fn parse_string_for_analysis(input: impl AsRef<str>) -> Result<Vec<Expr>, Vec<Error>> {
    let input = input.as_ref();

    let mut lexer = Lexer::new(input);
    let tokens = lexer.lex()?;

    let mut parser = Parser::for_analysis(&tokens);
    let exprs = parser.parse()?;

    Ok(exprs)
}
