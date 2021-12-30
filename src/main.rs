mod client;
use structopt::StructOpt;
use client::PiHoleClient;

#[derive(StructOpt, Debug)]
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
    let resp = client::PiHoleRestClient { hostname: config.pihole_hostname, password: config.pihole_password }.summary_raw();
    println!("{:#?}", resp);
}
