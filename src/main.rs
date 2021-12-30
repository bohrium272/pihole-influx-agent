mod client;
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

    #[structopt(short, long)]
    interval_seconds: u64,
}

fn main() {
    let config = Config::from_args();
    let client = client::PiHoleRestClient {
        hostname: config.pihole_hostname,
        password: config.pihole_password,
    };
    loop {
        println!("{:#?}", client.summary_raw());
        thread::sleep(Duration::from_secs(config.interval_seconds));
    }
}
