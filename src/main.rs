mod config;
mod influx;
mod pihole;
mod summary;

use config::Config;
use log::*;
use pihole::PiHoleClient;
use std::thread;
use std::time::Duration;
use clap::{App, Arg};

static VERSION: &str = "0.1.2";

fn main() {
    env_logger::init();
    debug!("Starting...");
    let app = App::new("PiHole Influx Agent")
                .version(VERSION)
                .about("Scrape a PiHole for metrics and dump them to InfluxDB")
                .author("bohrium272")
                .arg(Arg::with_name("config-file")
                     .short("c")
                     .long("config-file")
                     .value_name("FILE")
                     .help("Path to YAML configuration file, defaults to ./config.yaml")
                     .takes_value(true))
                .get_matches();
    let config = Config::from_file(app.value_of("config-file").unwrap_or("./config.yaml").to_string());
    let client = pihole::PiHoleRestClient {
        url: config.pihole_url,
        insecure: config.pihole_insecure,
    };
    debug!("Initialized PiHoleRestClient...");
    let influx_client = influx::InfluxClient {
        url: config.influx_url,
        token: config.influx_db_token,
        org_id: config.influx_db_org_id,
        insecure: config.influx_insecure,
    };
    debug!("Initialized InfluxClient...");
    loop {
        debug!("Fetching summary from PiHole");
        let raw_summary_result = client.summary_raw();
        if raw_summary_result.is_err() {
            error!("Error from PiHole: {}", raw_summary_result.unwrap_err());
            continue;
        }
        let raw_summary = raw_summary_result.unwrap();
        info!(
            "Received summary from PiHole: {} domains blocked today",
            raw_summary.ads_blocked_today
        );
        debug!("Writing to InfluxDB");
        let write_result = influx_client.write(config.influx_db_bucket.clone(), raw_summary);
        if write_result.is_err() {
            error!("Failed to publish metrics: {}", write_result.err().unwrap());
        }
        debug!("Waiting for {} seconds", config.interval_seconds);
        thread::sleep(Duration::from_secs(config.interval_seconds));
    }
}
