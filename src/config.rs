use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Config {
    #[structopt(short = "h", long)]
    pub pihole_hostname: String,

    #[structopt(short, long)]
    pub pihole_password: String,

    #[structopt(long)]
    pub pihole_https: bool,

    #[structopt(long)]
    pub pihole_insecure: bool,

    #[structopt(short, long, default_value = "30")]
    pub interval_seconds: u64,

    #[structopt(short = "d", long)]
    pub influx_db_host: String,

    #[structopt(short = "t", long)]
    pub influx_db_token: String,

    #[structopt(short = "b", long)]
    pub influx_db_bucket: String,

    #[structopt(short = "o", long)]
    pub influx_db_org_id: String,

    #[structopt(long)]
    pub influx_https: bool,

    #[structopt(long)]
    pub influx_insecure: bool,
}
