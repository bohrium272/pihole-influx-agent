mod client;
mod influx;

use client::PiHoleClient;
use std::thread;
use std::time::Duration;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Config {
    #[structopt(short = "h", long)]
    pihole_hostname: String,

    #[structopt(short, long)]
    pihole_password: String,

    #[structopt(long)]
    pihole_https: bool,

    #[structopt(long)]
    pihole_insecure: bool,

    #[structopt(short, long)]
    interval_seconds: u64,

    #[structopt(short = "d", long)]
    influx_db_host: String,

    #[structopt(short = "t", long)]
    influx_db_token: String,

    #[structopt(short = "b", long)]
    influx_db_bucket: String,

    #[structopt(short = "o", long)]
    influx_db_org_id: String,

    #[structopt(long)]
    influx_https: bool,

    #[structopt(long)]
    influx_insecure: bool,
}

fn main() {
    let config = Config::from_args();
    let client = client::PiHoleRestClient {
        hostname: config.pihole_hostname,
        password: config.pihole_password,
        https: config.pihole_https,
        insecure: config.pihole_insecure,
    };
    let influx_client = influx::InfluxClient {
        hostname: config.influx_db_host,
        token: config.influx_db_token,
        org_id: config.influx_db_org_id,
        https: config.influx_https,
        insecure: config.influx_insecure,
    };
    loop {
        let raw_summary = client.summary_raw();
        influx_client.write(config.influx_db_bucket.clone(), raw_summary);
        thread::sleep(Duration::from_secs(config.interval_seconds));
    }
}
