use crate::{keyword::Keyword, literal::Literal, operator::Operator, token::Token};
use std::{
    io::{Error, ErrorKind, Result},
    iter::Peekable,
    process,
    str::Chars,
};

pub struct Tokenizer;

impl Tokenizer {
    #[must_use]
    pub fn tokenize(text: &str) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut chars = text.chars().peekable();

        loop {
            let Some(current_char) = chars.next() else { break };

            let token = match (current_char, chars.peek()) {
                ('(', _) => Token::LParen,
                (')', _) => Token::RParen,
                ('{', _) => Token::LBrace,
                ('}', _) => Token::RBrace,
                ('.', _) => Token::Dot,
                ('+', _) => Token::Op(Operator::Add),
                ('-', _) => Token::Op(Operator::Sub),
                ('*', _) => Token::Op(Operator::Mul),
                ('/', _) => Token::Op(Operator::Div),
                ('%', _) => Token::Op(Operator::Mod),
                ('&', _) => Token::Op(Operator::And),
                ('|', _) => Token::Op(Operator::Or),
                ('>', Some(&'=')) => {
                    chars.next();
                    Token::Op(Operator::GreaterThanEqual)
                }
                ('<', Some(&'=')) => {
                    chars.next();
                    Token::Op(Operator::LessThanEqual)
                }
                ('=', Some(&'=')) => {
                    chars.next();
                    Token::Op(Operator::EqualEqual)
                }
                ('!', Some(&'=')) => {
                    chars.next();
                    Token::Op(Operator::NotEqual)
                }
                ('>', _) => Token::Op(Operator::GreaterThan),
                ('<', _) => Token::Op(Operator::LessThan),
                ('=', _) => Token::Op(Operator::Equal),
                (',', _) => Token::Comma,
                // (' ' | '\n' | '\t', _) => Token::Whitespace {
                //     value: current_char,
                // },
                (' ' | '\n' | '\t', _) => {
                    continue;
                }

                (unknown, _) => Tokenizer::parse_non_trivial_token(unknown, &mut chars)
                    .unwrap_or_else(|_| {
                        println!("Invalid Charater {unknown}");
                        process::exit(-1);
                    }),
            };

            tokens.push(token);
        }

        tokens
    }

    fn parse_non_trivial_token(unknown: char, chars: &mut Peekable<Chars>) -> Result<Token> {
        match unknown {
            '"' | '\'' => {
                let string_content = Tokenizer::parse_word(&['"', '\''], chars);
                chars.next();
                Ok(Token::Literal(Literal::String(string_content)))
            }

            ';' => {
                let string_content = Tokenizer::parse_word(&['\n'], chars);
                Ok(Token::Comment(string_content))
            }

            'a'..='z' | 'A'..='Z' | '_' => {
                let mut identifier = String::from(unknown);
                let word = Tokenizer::parse_word(
                    &[
                        ' ', '(', ')', '\n', '\t', ',', '-', '+', '=', '/', '*', '%', '.', '{',
                        '}', ';',
                    ],
                    chars,
                );

                identifier += &word;

                if let Some(keyword) = Tokenizer::parse_keyword(identifier.as_str()) {
                    Ok(keyword)
                } else {
                    Ok(Token::Identifier(identifier))
                }
            }

            '0'..='9' => {
                let mut number = String::from(unknown);
                let word = Tokenizer::parse_word(
                    &[
                        ' ', '(', ')', '\n', '\t', ',', '-', '+', '=', '/', '*', '%', '.', '{',
                        '}', ';',
                    ],
                    chars,
                );

                number += &word;

                match number.parse::<u32>() {
                    Ok(number) => Ok(Token::Literal(Literal::Number(number))),
                    Err(_) => Err(Error::from(ErrorKind::InvalidInput)),
                }
            }

            _ => Err(Error::from(ErrorKind::InvalidInput)),
        }
    }

    fn parse_keyword(content: &str) -> Option<Token> {
        match content {
            "ret" => Some(Token::Keyword(Keyword::Return)),
            "if" => Some(Token::Keyword(Keyword::If)),
            "else" => Some(Token::Keyword(Keyword::Else)),
            "set" => Some(Token::Keyword(Keyword::Set)),
            "func" => Some(Token::Keyword(Keyword::Func)),
            "print" => Some(Token::Keyword(Keyword::Print)),
            _ => None,
        }
    }

    fn parse_word(constraints: &[char], chars: &mut Peekable<Chars>) -> String {
        let mut result = String::new();

        while let Some(character) = chars.peek() {
            if constraints.iter().any(|&constrain| constrain == *character) {
                break;
            }

            result += &character.to_string();
            chars.next();
        }

        result
    }
}
