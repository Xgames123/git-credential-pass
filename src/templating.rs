use log::*;
use std::{collections::HashMap, ops::Range};

pub fn populate(template: &str, params: &HashMap<String, String>) -> String {
    let mut output = template.to_string();
    for (key, value) in params.iter() {
        // trace!("output={}", &output);
        loop {
            if let Some(range) = find_key(key, &output) {
                output.replace_range(range, value);
            } else {
                break;
            }
        }
    }

    output
}

pub fn get_params(template: &str, input: &str) -> HashMap<String, String> {
    let mut output = HashMap::new();

    for (i, character) in input {}

    output
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
