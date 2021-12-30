use structopt::StructOpt;

mod client;

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
    println!("{:#?}", config);
}
