use std::{
    collections::HashMap,
    io::{stdin, BufRead},
};

pub fn write_to_stdout(params: HashMap<String, String>) {
    for (key, value) in params.iter() {
        println!("{}={}", key, value);
    }
    println!("");
}

pub fn parse_from_stdin() -> HashMap<String, String> {
    let mut params = HashMap::<String, String>::new();
    let mut stdin = stdin().lock();
    loop {
        let mut buf = String::new();

        stdin
            .read_line(&mut buf)
            .expect("Failed to read line from stdin");
        let buf = buf.trim();
        if buf.is_empty() {
            return params;
        }
        if let Some((key, value)) = buf.split_once('=') {
            params.insert(key.to_string(), value.to_string());
        }
    }
}
