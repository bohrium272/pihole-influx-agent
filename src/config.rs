use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::BufReader;


#[derive(Serialize, Deserialize)]
pub struct Config {
    pub pihole_url: String,
    pub pihole_insecure: bool,

    pub interval_seconds: u64,

    pub influx_url: String,
    pub influx_db_token: String,
    pub influx_db_bucket: String,
    pub influx_db_org_id: String,
    pub influx_insecure: bool,
}

impl Config {
    pub fn from_file(path: String) -> Config {
        let f = File::open(path);
        let reader = BufReader::new(f.unwrap());

        serde_yaml::from_reader(reader).unwrap()
    }
}
