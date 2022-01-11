extern crate dirs;

use std::path::Path;

pub fn build_astromonitor_folder_tree() -> std::io::Result<()> {
    let folder_path = ".local/share/astromonitor";
    let logs_path = "logs";
    let home = dirs::home_dir().unwrap().as_path().display().to_string();
    let root_path = format!("{}/{}", home, folder_path);
    let logs_path = format!("{}/{}", root_path, logs_path);

    match Path::new(&root_path).exists() {
        true => (),
        false => std::fs::create_dir(&root_path)?,
    };
    match Path::new(&logs_path).exists() {
        true => (),
        false => std::fs::create_dir(&logs_path)?,
    }
    Ok(())
}
