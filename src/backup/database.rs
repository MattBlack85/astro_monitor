use astromonitor::Paths;
use minreq;
use std::fs::File;
use std::io::Read;
use tar::Builder;

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
        Err(e) => println!("Couldn't append indi folder to the archive, reason: {}", e),
    }

    // Add kstars database to the archive
    let mut db = match File::open(paths.db_full_path()) {
        Ok(f) => f,
        Err(e) => panic!("Couldn't open the Kstars database, reason: {}", e),
    };

    match arch.append_file("backup/kstars/userdb.sqlite", &mut db) {
        Ok(_) => (),
        Err(e) => println!("Couldn't append the database to the archive, reason: {}", e),
    }

    match arch.finish() {
        Ok(_) => (),
        Err(e) => println!("Couldn't create the archive, reason: {}", e),
    }

    let mut content = Vec::new();
    let mut f = match File::open("/tmp/k_backup.tar") {
        Ok(f) => f,
        Err(e) => panic!("Couldn't read the archive, reason: {}", e),
    };

    match f.read_to_end(&mut content) {
        Ok(_) => (),
        Err(_e) => println!("Error while reading backup"),
    }

    match minreq::post(format!("http://astromatto.com:11111/backup/db/{token}"))
        .with_header("content-type", "application/octet-stream")
        .with_body(content)
        .send()
    {
        Ok(r) => {
            println!("Request successful, response is {:?}", r);
            Ok(())
        }
        Err(e) => Err(format!("Request failed with reason {}", e)),
    }
}
