use std::{
    io::Write,
    process::{Command, Output, Stdio},
};

use log::*;

fn spawn_pass(cmd: &mut Command) -> std::process::Child {
    cmd.spawn()
        .expect("Failed to spawn pass process. Is it installed and added to path?")
}
fn wait_validate(proc: std::process::Child) -> Output {
    let output = proc.wait_with_output().unwrap();
    if !output.status.success() {
        error!(
            "Pass returned exit code {}\n{}",
            output.status.code().unwrap_or(-1),
            String::from_utf8_lossy(&output.stderr)
        );
    }
    output
}

pub fn remove_password(passname: &str) {
    let proc = spawn_pass(Command::new("pass").arg("rm").arg("-f").arg(passname));
    wait_validate(proc);
}

pub fn insert_password(passname: &str, data: &str) {
    let mut pass_proc = spawn_pass(
        Command::new("pass")
            .arg("insert")
            .arg("-m")
            .arg(passname)
            .stdin(Stdio::piped()),
    );

    let mut stdin = pass_proc.stdin.take().unwrap();
    stdin
        .write_all(data.as_bytes())
        .expect("Failed to write to stdin of pass process");
    drop(stdin);

    wait_validate(pass_proc);
}

pub fn get_password(passname: &str) -> String {
    let pass_proc = spawn_pass(Command::new("pass").arg("show").arg(passname));
    let output = wait_validate(pass_proc);

    String::from_utf8(output.stdout).unwrap_or_else(|err| {
        error!("Pass output contains invalid utf8 characters.");
        String::from_utf8_lossy(err.as_bytes()).into()
    })
}
