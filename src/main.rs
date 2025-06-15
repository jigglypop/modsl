use std::iter::Peekable;

// 1. AST (Abstract Syntax Tree) 정의
#[derive(Debug, Clone, PartialEq)]
pub enum Op {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Number(f64),
    BinaryOp(Box<Expr>, Op, Box<Expr>),
}

// 2. 토큰(Token) 정의
#[derive(Debug, Clone, PartialEq)]
enum Token {
    Number(f64),
    Op(Op),
    LParen,
    RParen,
}

// 3. 렉서 (문자열 -> 토큰 스트림)
fn lex(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            '0'..='9' | '.' => {
                let mut num_str = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_digit(10) || c == '.' {
                        num_str.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Number(num_str.parse().unwrap()));
            }
            '+' => {
                tokens.push(Token::Op(Op::Add));
                chars.next();
            }
            '-' => {
                tokens.push(Token::Op(Op::Subtract));
                chars.next();
            }
            '*' => {
                tokens.push(Token::Op(Op::Multiply));
                chars.next();
            }
            '/' => {
                tokens.push(Token::Op(Op::Divide));
                chars.next();
            }
            '(' => {
                tokens.push(Token::LParen);
                chars.next();
            }
            ')' => {
                tokens.push(Token::RParen);
                chars.next();
            }
            ' ' | '\t' | '\n' => {
                chars.next(); // 공백 무시
            }
            _ => {
                // 자연어 처리
                let mut word = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_alphabetic() {
                        word.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                match word.as_str() {
                    "plus" => tokens.push(Token::Op(Op::Add)),
                    "minus" => tokens.push(Token::Op(Op::Subtract)),
                    "times" => tokens.push(Token::Op(Op::Multiply)),
                    "divided" => {
                        // "divided by" 처리
                        chars.next(); // by
                        chars.next(); // 공백
                        tokens.push(Token::Op(Op::Divide))
                    },
                    _ => {} // 모르는 단어는 무시
                }
            }
        }
    }
    tokens
}

// 4. 재귀 하강 파서
fn parse_expr(tokens: &mut Peekable<std::slice::Iter<Token>>) -> Expr {
    parse_sum(tokens)
}

fn parse_sum(tokens: &mut Peekable<std::slice::Iter<Token>>) -> Expr {
    let mut left = parse_product(tokens);
    while let Some(Token::Op(op)) = tokens.peek() {
        match op {
            Op::Add | Op::Subtract => {
                let op = tokens.next().unwrap().clone();
                let op = match op {
                    Token::Op(op) => op,
                    _ => unreachable!(),
                };
                let right = parse_product(tokens);
                left = Expr::BinaryOp(Box::new(left), op, Box::new(right));
            }
            _ => break,
        }
    }
    left
}

fn parse_product(tokens: &mut Peekable<std::slice::Iter<Token>>) -> Expr {
    let mut left = parse_atom(tokens);
    while let Some(Token::Op(op)) = tokens.peek() {
        match op {
            Op::Multiply | Op::Divide => {
                let op = tokens.next().unwrap().clone();
                let op = match op {
                    Token::Op(op) => op,
                    _ => unreachable!(),
                };
                let right = parse_atom(tokens);
                left = Expr::BinaryOp(Box::new(left), op, Box::new(right));
            }
            _ => break,
        }
    }
    left
}

fn parse_atom(tokens: &mut Peekable<std::slice::Iter<Token>>) -> Expr {
    match tokens.next() {
        Some(Token::Number(n)) => Expr::Number(*n),
        Some(Token::LParen) => {
            let expr = parse_expr(tokens);
            assert_eq!(tokens.next(), Some(&Token::RParen));
            expr
        }
        _ => panic!("Unexpected token"),
    }
}


fn main() {
    let src = "5 times (3 plus 4) - 1";
    println!("MoDsl - A new era of mathematical notation.");
    println!("\nParsing: {}", src);

    let tokens = lex(src);
    println!("\nTokens: {:?}", tokens);

    let mut token_iter = tokens.iter().peekable();
    let ast = parse_expr(&mut token_iter);

    println!("\nParsed AST: {:?}", ast);
} 