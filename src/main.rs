mod config;
mod influx;
mod pihole;
mod summary;

use config::Config;
use env_logger;
use log::*;
use pihole::PiHoleClient;
use std::thread;
use std::time::Duration;
use structopt::StructOpt;

fn main() {
    env_logger::init();
    debug!("Starting...");
    let config = Config::from_args();
    let client = pihole::PiHoleRestClient {
        hostname: config.pihole_hostname,
        password: config.pihole_password,
        https: config.pihole_https,
        insecure: config.pihole_insecure,
    };
    debug!("Initialized PiHoleRestClient...");
    let influx_client = influx::InfluxClient {
        hostname: config.influx_db_host,
        token: config.influx_db_token,
        org_id: config.influx_db_org_id,
        https: config.influx_https,
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
