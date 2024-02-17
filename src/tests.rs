use crate::utils;

#[test]
fn path_home_dir() {
    assert_eq!(
        utils::abs_path("~/gcp_testdir/../gcp_testdir/testdir2"),
        format!("{}/gcp_testdir/testdir2", env!("HOME"))
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
