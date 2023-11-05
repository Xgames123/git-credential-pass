pub fn insert_password(passname: &str, data: &str) {}

pub fn get_password(passname: &str) -> String {
    let pass_output = Command::new("pass")
        .arg("show")
        .arg(passname)
        .output()
        .unwrap();
    if !pass_output.status.success() {
        error!(
            "Pass returned exit code {}",
            pass_output.status.code().unwrap_or(-1)
        );
    }

    String::from_utf8(pass_output.stdout).expect("Pass output contains invalid utf8 characters")
}
