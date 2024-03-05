use std::time;
use structopt::StructOpt;

pub static INTERVAL: time::Duration = time::Duration::from_secs(15);
pub static HOST: &'static str = "http://astromatto.com:11111";

#[derive(StructOpt)]
pub struct CliArgs {
    pub api_token: String,
    #[structopt(long)]
    pub fd_monitor: bool,
    #[structopt(long)]
    pub system_monitor: bool,
    #[structopt(long)]
    pub do_backup: bool,
    #[structopt(long)]
    pub retrieve_backup: bool,
    #[structopt(long)]
    pub kstars: bool,
}

pub struct Paths {
    folder_path: String,
    logs_path: String,
    pub home_path: String,
    db_path: String,
    city_db_path: String,
    indi_conf_path: String,
    fov_path: String,
}

impl Paths {
    pub fn logs_full_path(&self) -> String {
        let logs_full_path = format!("{}/{}", self.root_path(), self.logs_path);
        return logs_full_path;
    }

    pub fn root_path(&self) -> String {
        let root_path = format!("{}/{}", self.home_path, self.folder_path);
        return root_path;
    }

    pub fn db_full_path(&self) -> String {
        let db_path = format!("{}/{}", self.home_path, self.db_path);
        return db_path;
    }

    pub fn city_db_full_path(&self) -> String {
        let db_path = format!("{}/{}", self.home_path, self.city_db_path);
        return db_path;
    }

    pub fn fov_full_path(&self) -> String {
        let path = format!("{}/{}", self.home_path, self.fov_path);
        return path;
    }

    pub fn indi_conf_full_path(&self) -> String {
        let indi_conf_path = format!("{}/{}", self.home_path, self.indi_conf_path);
        return indi_conf_path;
    }

    pub fn init() -> Self {
        Self {
            folder_path: String::from(".local/share/astromonitor"),
            logs_path: String::from("logs"),
            home_path: dirs::home_dir().unwrap().as_path().display().to_string(),
            #[cfg(target_os = "linux")]
            db_path: String::from(".local/share/kstars/userdb.sqlite"),
            #[cfg(target_os = "macos")]
            db_path: String::from("Library/Application Support/kstars/userdb.sqlite"),
            indi_conf_path: String::from(".indi/"),
            city_db_path: String::from(".local/share/kstars/mycitydb.sqlite"),
            fov_path: String::from(".local/share/kstars/fov.dat"),
        }
    }
}
