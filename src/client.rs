use reqwest;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct SummaryRaw {
    pub domains_being_blocked: i64,
    pub dns_queries_today: i64,
    pub ads_blocked_today: i64,
    pub ads_percentage_today: f64,
    pub unique_domains: i64,
    pub queries_forwarded: i64,
    pub queries_cached: i64,
    pub clients_ever_seen: i64,
    pub unique_clients: i64,
    pub dns_queries_all_types: i64,
    pub privacy_level: i64,
    pub status: String,

    #[serde(rename(serialize = "reply_NODATA", deserialize = "reply_NODATA"))]
    pub reply_nodata: i64,

    #[serde(rename(serialize = "reply_NXDOMAIN", deserialize = "reply_NXDOMAIN"))]
    pub reply_nxdomain: i64,

    #[serde(rename(serialize = "reply_IP", deserialize = "reply_IP"))]
    pub reply_ip: i64,
}

pub trait PiHoleClient {
    fn summary_raw(&self) -> SummaryRaw;
}

pub struct PiHoleRestClient {
    pub hostname: String,
    pub password: String,
}

impl PiHoleClient for PiHoleRestClient {
    fn summary_raw(&self) -> SummaryRaw {
        let url = format!("http://{}/admin/api.php?summaryRaw", self.hostname);
        let resp = reqwest::blocking::get(url)
            .unwrap()
            .json::<SummaryRaw>()
            .unwrap();

        return resp;
    }
}
