use std::iter::Peekable;
use std::slice::Iter;

#[derive(Debug, PartialEq, Clone)]
pub enum ParseError {
    UnexpectedEndOfInput,
    UnexpectedToken(String),
    ExpectedClosingParen,
    ExpectedOperator,
    InvalidToken(String),
}

// Helper function to tokenize the input string
fn tokenize(input: &str) -> Vec<String> {
    input
        .replace('(', " ( ")
        .replace(')', " ) ")
        .split_whitespace()
        .map(String::from)
        .collect()
}

// Recursive parsing function
fn parse_recursive(tokens: &mut Peekable<Iter<String>>) -> Result<String, ParseError> {
    match tokens.next() {
        Some(token) => {
            if token == "(" {
                let operator_token = tokens.next().ok_or(ParseError::UnexpectedEndOfInput)?;
                let operator = operator_token.as_str();

                let infix_expr = match operator {
                    "+" | "*" => {
                        let left_operand = match tokens.peek() {
                            Some(next_token) if next_token.as_str() == ")" => {
                                return Err(ParseError::UnexpectedToken(
                                    "Missing left operand before ')'".to_string(),
                                ));
                            }
                            None => return Err(ParseError::UnexpectedEndOfInput),
                            _ => parse_recursive(tokens)?,
                        };

                        let right_operand = match tokens.peek() {
                            Some(next_token) if next_token.as_str() == ")" => {
                                return Err(ParseError::UnexpectedToken(
                                    "Missing right operand before ')'".to_string(),
                                ));
                            }
                            None => return Err(ParseError::UnexpectedEndOfInput),
                            _ => parse_recursive(tokens)?,
                        };
                        format!("({} {} {})", left_operand, operator, right_operand)
                    }
                    "Vec" => { // Assuming Vec is still a possibility based on previous code
                        let operand = match tokens.peek() {
                            Some(next_token) if next_token.as_str() == ")" => {
                                return Err(ParseError::UnexpectedToken(
                                    "Missing operand for Vec before ')'".to_string(),
                                ));
                            }
                            None => return Err(ParseError::UnexpectedEndOfInput),
                            _ => parse_recursive(tokens)?,
                        };
                        format!("Vec({})", operand)
                    }
                    _ => {
                        return Err(ParseError::ExpectedOperator);
                    }
                };

                match tokens.next() {
                    Some(close_paren) if close_paren == ")" => Ok(infix_expr),
                    Some(other) => Err(ParseError::UnexpectedToken(format!(
                        "Expected ')' but got '{}' after expression for operator '{}'",
                        other, operator
                    ))),
                    None => Err(ParseError::ExpectedClosingParen),
                }
            } else {
                // Atom (identifier)
                if token.starts_with('(') || token.starts_with(')') || token.contains(char::is_whitespace) || token.is_empty() {
                     Err(ParseError::InvalidToken(token.clone()))
                } else {
                    Ok(token.clone())
                }
            }
        }
        None => Err(ParseError::UnexpectedEndOfInput),
    }
}

pub fn prefix_to_infix_str(prefix_expr: &str) -> Result<String, ParseError> {
    let tokens = tokenize(prefix_expr);
    if tokens.is_empty() {
        return Err(ParseError::UnexpectedEndOfInput);
    }
    let mut tokens_iter = tokens.iter().peekable();
    let result = parse_recursive(&mut tokens_iter)?;

    if tokens_iter.next().is_some() {
        Err(ParseError::UnexpectedToken(
            "Extra tokens found after parsing".to_string(),
        ))
    } else {
        Ok(result)
    }
}
