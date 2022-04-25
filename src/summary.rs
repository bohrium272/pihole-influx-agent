use serde::Deserialize;
use std::fmt::Display;

use crate::influx::InfluxMetric;

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

impl Display for SummaryRaw {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}% requests blocked today!", self.ads_percentage_today)
    }
}

impl InfluxMetric for SummaryRaw {
    fn influx_metric(self) -> String {
        let hostname = sys_info::hostname().unwrap();
        let mut metric = "".to_owned();
        metric.push_str(
            &format!(
                "{},hostname={} last={}\n",
                "domains_being_blocked", hostname, self.domains_being_blocked
            )
            .to_string(),
        );
        metric.push_str(
            &format!(
                "{},hostname={} last={}\n",
                "dns_queries_today", hostname, self.dns_queries_today
            )
            .to_string(),
        );
        metric.push_str(
            &format!(
                "{},hostname={} last={}\n",
                "ads_blocked_today", hostname, self.ads_blocked_today
            )
            .to_string(),
        );
        metric.push_str(
            &format!(
                "{},hostname={} last={}\n",
                "ads_percentage_today", hostname, self.ads_percentage_today
            )
            .to_string(),
        );
        metric.push_str(
            &format!(
                "{},hostname={} last={}\n",
                "unique_domains", hostname, self.unique_domains
            )
            .to_string(),
        );
        metric.push_str(
            &format!(
                "{},hostname={} last={}\n",
                "queries_forwarded", hostname, self.queries_forwarded
            )
            .to_string(),
        );
        metric.push_str(
            &format!(
                "{},hostname={} last={}\n",
                "queries_cached", hostname, self.queries_cached
            )
            .to_string(),
        );
        metric.push_str(
            &format!(
                "{},hostname={} last={}\n",
                "clients_ever_seen", hostname, self.clients_ever_seen
            )
            .to_string(),
        );
        metric.push_str(
            &format!(
                "{},hostname={} last={}\n",
                "unique_clients", hostname, self.unique_clients
            )
            .to_string(),
        );
        metric.push_str(
            &format!(
                "{},hostname={} last={}\n",
                "dns_queries_all_types", hostname, self.dns_queries_all_types
            )
            .to_string(),
        );
        metric.push_str(
            &format!(
                "{},hostname={} last={}\n",
                "privacy_level", hostname, self.privacy_level
            )
            .to_string(),
        );
        let status_bool = if self.status == "enabled" { "t" } else { "f" };
        metric.push_str(
            &format!("{},hostname={} last={}", "status", hostname, status_bool).to_string(),
        );
        return metric;
    }
}
