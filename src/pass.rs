use log::*;
use std::{
    error::Error,
    io::Write,
    process::{Command, Output, Stdio},
};
use thiserror::Error;

fn spawn_pass(cmd: &mut Command) -> Result<std::process::Child, PassError> {
    Ok(cmd.spawn()?)
}
fn wait_validate(proc: std::process::Child) -> Result<Output, PassError> {
    let output = proc.wait_with_output().unwrap();
    if !output.status.success() {
        return Err(PassError::Non0ExitCode(
            output.status.code().unwrap_or_default(),
            String::from_utf8_lossy(&output.stderr).to_string(),
        ));
    }
    Ok(output)
}

macro_rules! pass_cmd {
    () => {
        Command::new("pass")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
    };
}

#[derive(Error, Debug)]
pub enum PassError {
    #[error("{0}")]
    Io(#[from] std::io::Error),
    #[error("Pass exited with code: {0}\n{1}")]
    Non0ExitCode(i32, String),
}

pub fn remove_password(passname: &str) -> Result<(), PassError> {
    let proc = spawn_pass(pass_cmd!().arg("rm").arg("-f").arg(passname))?;
    wait_validate(proc)?;
    Ok(())
}

pub fn insert_password(passname: &str, data: &str) -> Result<(), PassError> {
    let mut pass_proc = spawn_pass(pass_cmd!().arg("insert").arg("-m").arg(passname))?;

    let mut stdin = pass_proc.stdin.take().unwrap();
    stdin
        .write_all(data.as_bytes())
        .expect("Failed to write to stdin of pass process");
    drop(stdin);

    wait_validate(pass_proc)?;
    Ok(())
}

pub fn get_password(passname: &str) -> Result<String, PassError> {
    let pass_proc = spawn_pass(pass_cmd!().arg("show").arg(passname))?;
    let output = wait_validate(pass_proc)?;

    Ok(String::from_utf8(output.stdout).unwrap_or_else(|err| {
        error!("Pass output contains invalid utf8 characters.");
        String::from_utf8_lossy(err.as_bytes()).into()
    }))
}
