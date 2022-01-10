use std::process::Command;

pub fn lsof_on_system() -> bool {
    let result = Command::new("which").arg("lsof").output().expect("error");
    let output = String::from_utf8(result.stdout)
        .expect("fail to parse output")
        .to_string();
    match output.as_ref() {
        "" => return false,
        _ => return true,
    };
}
