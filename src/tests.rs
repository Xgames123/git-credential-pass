use crate::utils;

#[test]
fn path_home_dir() {
    assert_eq!(
        utils::abs_path("~/.config/../.config/git-credential-pass"),
        format!("{}/.config/git-credential-pass", env!("HOME"))
    )
}

#[test]
fn path_cwd_dir() {
    assert_eq!(
        utils::abs_path("testing/templates/../test.sh"),
        format!(
            "{}/testing/test.sh",
            std::env::current_dir().unwrap().to_str().unwrap()
        )
    )
}

#[test]
fn path_root_dir() {
    assert_eq!(utils::abs_path("/etc/../usr/local"), "/usr/local")
}

#[test]
fn path_outside_home() {
    assert_eq!(utils::abs_path("~/../../usr"), "/usr")
}
