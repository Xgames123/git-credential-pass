use std::fs;

use log::*;

#[macro_export()]
macro_rules! die {
    ($($arg:tt)+) => {log::error!("FATAL: {}", format!($($arg)+)); std::process::exit(1);}
}

pub fn abs_path(path: &str) -> String {
    let exp_path = shellexpand::full(path).unwrap_or_else(|err| {
        error!("Could not expand path '{}'\n{}", path, err);
        path.into()
    });
    fs::canonicalize(exp_path.as_ref())
        .map(|pth| pth.to_string_lossy().to_string())
        .unwrap_or_else(|err| {
            error!("Could not expand path '{}'\n{}", path, err);
            path.to_string()
        })
}
