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
    pub kstars: bool,
}

pub struct Paths {
    folder_path: String,
    logs_path: String,
    home_path: String,
    db_path: String,
    indi_conf_path: String,
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

    pub fn indi_conf_full_path(&self) -> String {
        let indi_conf_path = format!("{}/{}", self.home_path, self.indi_conf_path);
        return indi_conf_path;
    }

    pub fn init() -> Self {
        Self {
            folder_path: String::from(".local/share/astromonitor"),
            logs_path: String::from("logs"),
            home_path: dirs::home_dir().unwrap().as_path().display().to_string(),
            db_path: String::from(".local/share/kstars/userdb.sqlite"),
            indi_conf_path: String::from(".indi/"),
        }
    }
}