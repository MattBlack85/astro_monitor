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
    home_path: String,
    pub db_path: String,
    pub city_db_path: String,
    pub indi_conf_path: String,
    pub fov_path: String,
    pub kstars_rc_path: String,
    pub phd2_profile_path: String,
    pub phd2_filename: String,
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

    pub fn init() -> Self {
        let home_path = dirs::home_dir().unwrap().as_path().display().to_string();
        let config_path = dirs::preference_dir()
            .unwrap()
            .as_path()
            .display()
            .to_string();
        let data_path = dirs::data_dir().unwrap().as_path().display().to_string();

        #[cfg(target_os = "linux")]
        let phd2_filename = String::from(".PHDGuidingV2");

        #[cfg(target_os = "macos")]
        let phd2_filename = String::from("PHDGuidingV2_Preferences");

        Self {
            folder_path: String::from(".local/share/astromonitor"),
            logs_path: String::from("logs"),
            home_path: home_path.clone(),

            db_path: format!("{}/kstars/userdb.sqlite", &data_path),
            city_db_path: format!("{}/kstars/mycitydb.sqlite", &data_path),
            fov_path: format!("{}/kstars/fov.dat", &data_path),
            indi_conf_path: format!("{}/.indi/", &home_path),
            kstars_rc_path: format!("{}/kstarsrc", &config_path),
            phd2_filename: phd2_filename.clone(),
            #[cfg(target_os = "linux")]
            phd2_profile_path: format!("{}/{}", &home_path, &phd2_filename),
            #[cfg(target_os = "macos")]
            phd2_profile_path: format!("{}/{}", &config_path, &phd2_filename),
        }
    }
}
