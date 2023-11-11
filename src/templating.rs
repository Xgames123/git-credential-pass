use log::*;
use std::{collections::HashMap, ops::Range};
use thiserror::Error;

#[derive(Debug, Clone)]
pub enum Token {
    Capture(String),
    Match(char),
}
impl Token {
    fn is_capture(&self) -> bool {
        match self {
            Token::Capture(_) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Error)]
pub enum TemplatingError {
    #[error("Template token {0:?} can not be followed by {1:?}")]
    InvalidTokenOrder(Token, Token),
}

pub type SyntaxTree = Vec<Token>;

pub fn parse(str: &str) -> Result<SyntaxTree, TemplatingError> {
    let mut output = vec![];

    let mut cap_name: String = String::new();
    let mut in_cap_name = false;
    let mut escape = false;
    for char in str.chars() {
        if char == '\\' {
            escape = true;
            continue;
        }
        if !escape && char == '{' {
            in_cap_name = true;
            continue;
        }
        if in_cap_name {
            if char == '}' && !escape {
                let token = Token::Capture(cap_name.clone());
                if output.last().is_some_and(|last: &Token| last.is_capture()) {
                    return Err(TemplatingError::InvalidTokenOrder(
                        output.last().unwrap().clone(),
                        token,
                    ));
                }
                output.push(token);
                cap_name.clear();
                in_cap_name = false;
            } else {
                cap_name.push(char);
            }
            escape = false;
            continue;
        }

        output.push(Token::Match(char));
        escape = false;
    }

    Ok(output)
}

pub fn populate(template: &SyntaxTree, params: &HashMap<String, String>) -> String {
    let mut output = String::with_capacity(template.len());

    for token in template.iter() {
        match token {
            Token::Match(char) => output.push(*char),
            Token::Capture(name) => output.push_str(params.get(name).unwrap_or(&String::new())),
        }
    }

    output
}

pub fn get_params(
    template: &SyntaxTree,
    input: &str,
) -> Result<HashMap<String, String>, TemplatingError> {
    let mut output = HashMap::new();

    let mut iter = input.chars();
    let mut template_iter = template.iter();
    loop {
        let token = if let Some(token) = template_iter.next() {
            token
        } else {
            break;
        };
        match token {
            Token::Match(char) => loop {
                if let Some(temp_char) = iter.next() {
                    if temp_char == *char {
                        break;
                    }
                } else {
                    break;
                }
            },
            Token::Capture(capture) => {
                if let Some(next_token) = template_iter.next() {
                    match next_token {
                        Token::Match(end_character) => {
                            solve_capture(&capture, Some(*end_character), &mut iter, &mut output);
                        }
                        next_token => {
                            return Err(TemplatingError::InvalidTokenOrder(
                                token.clone(),
                                next_token.clone(),
                            ));
                        }
                    }
                } else {
                    solve_capture(&capture, None, &mut iter, &mut output);
                    break;
                }
            }
        }
    }

    Ok(output)
}

fn solve_capture(
    capture_name: &str,
    end_character: Option<char>,
    iter: &mut impl Iterator<Item = char>,
    output: &mut HashMap<String, String>,
) {
    let mut captured = String::new();
    loop {
        if let Some(char) = iter.next() {
            if end_character
                .map(|end_char| char == end_char)
                .unwrap_or(false)
            {
                break;
            }
            captured.push(char);
        } else {
            break;
        }
    }
    output.insert(capture_name.to_string(), captured);
}

fn find_key(key: &str, haystack: &str) -> Option<Range<usize>> {
    let key_len = key.chars().count();
    // trace!("key={}", key);
    // trace!("haystack={}", haystack);
    let mut match_count = 0;
    let mut start_index = 0;
    for (i, character) in haystack.char_indices() {
        // trace!("match_count={}, char={}", match_count, character);
        if match_count == 0 {
            if character == '{' {
                start_index = i;
                match_count += 1;
            }
            continue;
        }

        if match_count == key_len + 1 {
            // trace!("end");
            if character == '}' {
                return Some(start_index..i + 1);
            }
            match_count = 0;
            continue;
        }

        // trace!("matching chars");
        match get_char(key, match_count - 1) {
            Some(char) => {
                // trace!("{}={}", character, char);
                if character == char {
                    match_count += 1
                } else {
                    match_count = 0
                }
            }
            None => match_count = 0,
        }
    }

    None
}

fn get_char(input: &str, start: usize) -> Option<char> {
    input
        .get(char_start_to_range(input, start))
        .and_then(|str| str.chars().next())
}

///Returns the range that where the code point is in given the starting index
fn char_start_to_range(str: &str, start: usize) -> Range<usize> {
    for (i, _) in str.as_bytes()[start..].iter().enumerate() {
        if i == 0 {
            continue;
        }
        if str.is_char_boundary(start + i) {
            // trace!("{}..{}", start, start + i);
            return start..start + i;
        }
    }
    return start..str.len();
}
