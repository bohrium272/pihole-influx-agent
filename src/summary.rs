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
            ),
        );
        metric.push_str(
            &format!(
                "{},hostname={} last={}\n",
                "dns_queries_today", hostname, self.dns_queries_today
            ), 
        );
        metric.push_str(
            &format!(
                "{},hostname={} last={}\n",
                "ads_blocked_today", hostname, self.ads_blocked_today
            ),
        );
        metric.push_str(
            &format!(
                "{},hostname={} last={}\n",
                "ads_percentage_today", hostname, self.ads_percentage_today
            ),
        );
        metric.push_str(
            &format!(
                "{},hostname={} last={}\n",
                "unique_domains", hostname, self.unique_domains
            ),
        );
        metric.push_str(
            &format!(
                "{},hostname={} last={}\n",
                "queries_forwarded", hostname, self.queries_forwarded
            ),
        );
        metric.push_str(
            &format!(
                "{},hostname={} last={}\n",
                "queries_cached", hostname, self.queries_cached
            ),
        );
        metric.push_str(
            &format!(
                "{},hostname={} last={}\n",
                "clients_ever_seen", hostname, self.clients_ever_seen
            ),
        );
        metric.push_str(
            &format!(
                "{},hostname={} last={}\n",
                "unique_clients", hostname, self.unique_clients
            ),
        );
        metric.push_str(
            &format!(
                "{},hostname={} last={}\n",
                "dns_queries_all_types", hostname, self.dns_queries_all_types
            ),
        );
        metric.push_str(
            &format!(
                "{},hostname={} last={}\n",
                "privacy_level", hostname, self.privacy_level
            ),
        );
        let status_bool = if self.status == "enabled" { "t" } else { "f" };
        metric.push_str(
            &format!("{},hostname={} last={}", "status", hostname, status_bool),
        );
        metric
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_influx_metric() {
        let influx_metric = SummaryRaw {
            domains_being_blocked: 10000,
            dns_queries_today: 1000,
            ads_blocked_today: 100,
            ads_percentage_today: 10.5,
            unique_domains: 100,
            queries_forwarded: 900,
            queries_cached: 450,
            clients_ever_seen: 42,
            unique_clients: 42,
            dns_queries_all_types: 1000,
            privacy_level: 1,
            status: "OK".to_string(),
            reply_nodata: 333,
            reply_nxdomain: 333,
            reply_ip: 334
        };

        let metric_string = influx_metric.influx_metric();

        assert_eq!("domains_being_blocked,hostname=ranger last=10000\ndns_queries_today,hostname=ranger last=1000\nads_blocked_today,hostname=ranger last=100\nads_percentage_today,hostname=ranger last=10.5\nunique_domains,hostname=ranger last=100\nqueries_forwarded,hostname=ranger last=900\nqueries_cached,hostname=ranger last=450\nclients_ever_seen,hostname=ranger last=42\nunique_clients,hostname=ranger last=42\ndns_queries_all_types,hostname=ranger last=1000\nprivacy_level,hostname=ranger last=1\nstatus,hostname=ranger last=f".to_string(), metric_string);
    }
}
