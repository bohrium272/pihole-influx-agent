use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Config {
    #[structopt(env, about = "PiHole's IP/Hostname")]
    pub pihole_hostname: String,

    #[structopt(
        long,
        about = "Boolean flag that determines whether the PiHole API will be called over https"
    )]
    pub pihole_https: bool,

    #[structopt(
        long,
        about = "Boolean flag indicating whether to ignore certificate validation for the PiHole API call when done over HTTPS"
    )]
    pub pihole_insecure: bool,

    #[structopt(
        short,
        long,
        default_value = "30",
        about = "Scrape interval in seconds"
    )]
    pub interval_seconds: u64,

    #[structopt(env, about = "Influx DB URL ({protocol}://{host})")]
    pub influx_url: String,

    #[structopt(
        env,
        about = "Influx DB Token, must have write access to the specified bucket. https://docs.influxdata.com/influxdb/v2.1/security/tokens/create-token"
    )]
    pub influx_db_token: String,

    #[structopt(env, about = "Influx DB Bucket where the metrics will be stored")]
    pub influx_db_bucket: String,

    #[structopt(env, about = "The org ID to which this token belongs")]
    pub influx_db_org_id: String,

    #[structopt(
        long,
        about = "Boolean flag that determines whether to ignore certificate validation for the InfluxDB API call when done over https"
    )]
    pub influx_insecure: bool,
}
