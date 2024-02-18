use log::*;
use std::{
    io::Write,
    process::{Command, Output, Stdio},
};
use thiserror::Error;

fn spawn_pass(cmd: &mut Command) -> Result<std::process::Child, PassError> {
    Ok(cmd.spawn()?)
}
fn wait_validate(proc: std::process::Child) -> Result<Output, PassError> {
    let output = proc.wait_with_output()?;
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

fn retry_pass(cmd: &mut Command, retries: u32) -> Result<Output, PassError> {
    let mut output = None;
    for i in 0..retries + 1 {
        let pass_proc = spawn_pass(cmd)?;
        output = Some(match wait_validate(pass_proc) {
            Ok(o) => o,
            Err(err) => {
                if i == retries {
                    return Err(err);
                }
                error!("{} Retrying {}/{}", err, i, retries);
                continue;
            }
        });
        break;
    }
    Ok(output.unwrap()) //loop guarantied to run at least once
}

pub fn remove_password(passname: &str, retries: u32) -> Result<(), PassError> {
    debug!("removing password");
    retry_pass(pass_cmd!().arg("rm").arg("-f").arg(passname), retries)?;
    Ok(())
}

pub fn insert_password(passname: &str, data: &str, retries: u32) -> Result<(), PassError> {
    debug!("inserting password");
    for i in 0..retries + 1 {
        let mut pass_proc = match spawn_pass(pass_cmd!().arg("insert").arg("-m").arg(passname)) {
            Ok(proc) => proc,
            Err(err) => {
                if i == retries {
                    return Err(err);
                }
                error!("{} Retrying {}/{}", err, i, retries);
                continue;
            }
        };

        let mut stdin = pass_proc.stdin.take().unwrap();
        stdin
            .write_all(data.as_bytes())
            .expect("Failed to write to stdin of pass process");
        drop(stdin);

        match wait_validate(pass_proc) {
            Ok(_) => {}
            Err(err) => {
                if i == retries {
                    return Err(err);
                }
                error!("{} Retrying {}/{}", err, i, retries);
                continue;
            }
        };
        break;
    }

    Ok(())
}

pub fn get_password(passname: &str, retries: u32) -> Result<String, PassError> {
    debug!("getting password");
    let output = retry_pass(pass_cmd!().arg("show").arg(passname), retries)?;

    Ok(String::from_utf8(output.stdout).unwrap_or_else(|err| {
        error!("Pass output contains invalid utf8 characters.");
        String::from_utf8_lossy(err.as_bytes()).into()
    }))
}
