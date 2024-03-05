use astromonitor::Paths;
use log::info;
use minreq;
use std::fs::File;
use std::io::{Read, Write};
use tar::{Archive, Builder};

pub fn send_db(paths: &Paths, token: &String) -> Result<(), String> {
    // Init the zip archive
    let file = match File::create("/tmp/k_backup.tar") {
        Ok(f) => f,
        Err(e) => panic!("Couldn't create the tar archive, reason: {}", e),
    };

    let mut arch = Builder::new(file);

    // Add all indi devices xml configs to the archive
    match arch.append_dir_all("backup/indi", paths.indi_conf_full_path()) {
        Ok(_) => (),
        Err(e) => panic!("Couldn't append indi folder to the archive, reason: {}", e),
    }

    // Add kstars database to the archive
    let mut db = match File::open(paths.db_full_path()) {
        Ok(f) => f,
        Err(e) => panic!("Couldn't open the Kstars database, reason: {}", e),
    };

    match arch.append_file("backup/kstars/userdb.sqlite", &mut db) {
        Ok(_) => (),
        Err(e) => panic!("Couldn't append the database to the archive, reason: {}", e),
    }

    // Add city database to the archive
    let mut city_db = match File::open(paths.city_db_full_path()) {
        Ok(f) => f,
        Err(e) => panic!("Couldn't open the Kstars city database, reason: {}", e),
    };

    match arch.append_file("backup/kstars/mycitydb.sqlite", &mut city_db) {
        Ok(_) => (),
        Err(e) => panic!("Couldn't append the database to the archive, reason: {}", e),
    }

    // Add fov.dat to the archive
    let mut fov_db = match File::open(paths.fov_full_path()) {
        Ok(f) => f,
        Err(e) => panic!("Couldn't open the FOV database, reason: {}", e),
    };

    match arch.append_file("backup/kstars/fov.dat", &mut fov_db) {
        Ok(_) => (),
        Err(e) => panic!("Couldn't append the database to the archive, reason: {}", e),
    }

    match arch.finish() {
        Ok(_) => (),
        Err(e) => panic!("Couldn't create the archive, reason: {}", e),
    }

    let mut content = Vec::new();
    let mut f = match File::open("/tmp/k_backup.tar") {
        Ok(f) => f,
        Err(e) => panic!("Couldn't read the archive, reason: {}", e),
    };

    match f.read_to_end(&mut content) {
        Ok(_) => (),
        Err(_e) => panic!("Error while reading backup"),
    }

    match minreq::post(format!("http://astromatto.com:11111/backup/db/{token}"))
        .with_header("content-type", "application/octet-stream")
        .with_body(content)
        .send()
    {
        Ok(_) => {
            info!("Backup successful, you can have a peaceful sleep now!");
            Ok(())
        }
        Err(e) => Err(format!("Request failed with reason {}", e)),
    }
}

pub fn retrieve_db(paths: &Paths, token: &String) -> Result<(), String> {
    match minreq::get(format!("http://astromatto.com:11111/backup/db/{token}"))
        .with_header("content-type", "application/octet-stream")
        .send()
    {
        Ok(r) => {
            // Just make sure ~/.indi exists
            match std::fs::create_dir(format!("{}/{}", paths.home_path, ".indi")) {
                Ok(_) => (),
                Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => (),
                Err(e) => panic!("IO error: {}", e),
            };

            let mut f = File::create("temp_backup.tar").unwrap();
            f.write_all(r.as_bytes()).unwrap();
            let mut arch = Archive::new(File::open("temp_backup.tar").unwrap());
            for entry in arch.entries().unwrap() {
                let mut tf = entry.unwrap();
                let path = tf.path().unwrap();

                if &path.file_name().unwrap().to_str().unwrap() == &"indi" {
                    continue;
                }

                let mut s = Vec::new();
                let full_path: String;

                if path.to_str().unwrap().contains(&"indi") {
                    full_path = format!(
                        "{}{}",
                        paths.indi_conf_full_path(),
                        &path.file_name().unwrap().to_str().unwrap()
                    );
                } else if path.to_str().unwrap().contains(&"mycity") {
                    full_path = paths.city_db_full_path();
                } else if path.to_str().unwrap().contains(&"fov") {
                    full_path = paths.fov_full_path();
                } else {
                    full_path = paths.db_full_path();
                };

                tf.read_to_end(&mut s).unwrap();
                let mut f = File::create(full_path).unwrap();
                f.write(&s).unwrap();
            }
            std::fs::remove_file("temp_backup.tar").unwrap();
            info!("Backup restored with success");
            Ok(())
        }
        Err(e) => Err(format!("Request failed with reason {}", e)),
    }
}
