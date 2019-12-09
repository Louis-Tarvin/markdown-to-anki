use crate::card::*;

use std::fmt;
/// An enum for the different attributes a cards may have
pub enum Attribute {
    Front,
    Back,
    MainTag,
    SubTag,
}

/// An enum for the different types of error that could occur while parsing the document
pub enum ParseError {
    /// When the parse function encounters a character it wasn't expecting.
    /// Holds the line number and the character that caused the error
    UnknownSymbol((usize, char)),
    /// When the parse function encounters the end of the line before expecting it.
    /// Holds the line number where the error occured
    UnexpectedEndOfLine(usize),
    /// When the parse function encounters an unknown attruibute.
    /// Holds the line number where the error occured
    UnknownAttribute(usize),
}
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::UnknownSymbol(l) => {
                write!(f, "Unknown symbol: {} on line {}", l.1, l.0)
            },
            ParseError::UnexpectedEndOfLine(l) => {
                write!(f, "Unexpected end of line at line {}", l)
            },
            ParseError::UnknownAttribute(l) => {
                write!(f, "Unknown attribute encountered on line {}", l)
            }
        }
    }
}

#[derive(PartialEq)]
enum Type {
    Question,
    Definition,
}

pub fn parse_md(markdown: &str) -> Result<Vec<Card>, ParseError> {
    let mut cards:Vec<Card> = Vec::new();
    let mut main_tag = "";
    let mut sub_tag = "";
    let mut current_type = None;

    for (line_num, line) in markdown.lines().enumerate() {
        let parsed_line = parse_line(line, line_num, current_type)?;
        let attribute = parsed_line.0;
        let value = parsed_line.1;
        current_type = parsed_line.2;
        match attribute {
            Some(Attribute::Front) => {
                let tag = main_tag.replace(" ", "_").replace(",", "") + " " +
                          &sub_tag.replace(" ", "_").replace(",", "");
                if current_type == Some(Type::Definition) {
                    cards.push(Card::new("Define: ".to_string() + value.unwrap(),
                     "".to_string(), tag));
                } else {
                    cards.push(Card::new(value.unwrap().to_string(), "".to_string(), tag));
                }
            },
            Some(Attribute::Back) => {
                if let Some(c) = cards.last_mut() {
                    c.add_to_back(&value.unwrap());
                }
            },
            Some(Attribute::MainTag) => { main_tag = &value.unwrap(); },
            Some(Attribute::SubTag) => { sub_tag = &value.unwrap(); },
            None => {}
        }
    }

    Ok(cards)
}

type ParsedLine<'a> = Result<(Option<Attribute>, Option<&'a str>, Option<Type>), ParseError>;
fn parse_line(line: &str, line_num: usize, card_type: Option<Type>) -> ParsedLine {
    let mut line_iterator = line.chars();
    match card_type {
        Some(Type::Question) => {
            match line_iterator.next() {
                Some('-') => { Ok((Some(Attribute::Front), Some(&line[2..]), Some(Type::Question))) },
                Some(' ') => { Ok((Some(Attribute::Back), Some(&line[4..]), Some(Type::Question))) },
                Some(x) => { Err(ParseError::UnknownSymbol((line_num, x))) },
                None => { Ok((None, None, None)) },
            }
        },
        Some(Type::Definition) => {
            match line_iterator.next() {
                Some('-') => {
                    for (i, char) in line_iterator.enumerate() {
                        if char == '*' && i > 4 {
                            return Ok((Some(Attribute::Front), Some(&line[4..=i]), Some(Type::Definition)))
                        }
                    }
                    Err(ParseError::UnexpectedEndOfLine(line_num))
                },
                Some(_) => { Ok((Some(Attribute::Back), Some(&line), None))},
                None => { Err(ParseError::UnexpectedEndOfLine(line_num)) }
            }
        },
        None => {
            match line_iterator.next() {
                Some('#') => {
                    match line_iterator.next() {
                        Some(' ') => { Ok((Some(Attribute::MainTag), Some(&line[2..]), None)) },
                        Some('#') => {
                            for (i, char) in line_iterator.enumerate() {
                                if char == ' ' {
                                    return Ok((Some(Attribute::SubTag), Some(&line[i+3..]), None))
                                }
                            }
                            Err(ParseError::UnexpectedEndOfLine(line_num))
                        },
                        Some(x) => { Err(ParseError::UnknownSymbol((line_num, x))) },
                        None => { Err(ParseError::UnexpectedEndOfLine(line_num))},
                    }
                },
                Some('[') => {
                    for (i, char) in line_iterator.enumerate() {
                        if char == ')' {
                            match &line[3..=i] {
                                "question" => { return Ok((None, None, Some(Type::Question)))},
                                "definition" => { return Ok((None, None, Some(Type::Definition)))},
                                _ => { return Err(ParseError::UnknownAttribute(line_num)) }
                            }
                        }
                    }
                    Err(ParseError::UnexpectedEndOfLine(line_num))
                },
                _ => { Ok((None, None, None)) }
            }
        }
    }
}
