use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Config {
    #[structopt(env)]
    pub pihole_hostname: String,

    #[structopt(long)]
    pub pihole_https: bool,

    #[structopt(long)]
    pub pihole_insecure: bool,

    #[structopt(short, long, default_value = "30")]
    pub interval_seconds: u64,

    #[structopt(env)]
    pub influx_db_host: String,

    #[structopt(env)]
    pub influx_db_token: String,

    #[structopt(env)]
    pub influx_db_bucket: String,

    #[structopt(env)]
    pub influx_db_org_id: String,

    #[structopt(long)]
    pub influx_https: bool,

    #[structopt(long)]
    pub influx_insecure: bool,
}
