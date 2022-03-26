use minreq;
use std::error::Error;
use std::fs::File;
use std::io::Read;

pub fn send_db(db_path: String) -> Result<(), Box<dyn Error>> {
    let mut file = File::open(db_path)?;
    let mut s = String::new();
    file.read_to_string(&mut s);
    let r = minreq::post("http://127.0.0.1:5000").with_body(s).send();
    println!("{:?}", r);

    Ok(())
}
