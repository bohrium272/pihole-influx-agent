use crate::summary::SummaryRaw;
use reqwest::blocking::Client;
use reqwest::Error;

pub trait PiHoleClient {
    fn summary_raw(&self) -> Result<SummaryRaw, Error>;
}

pub struct PiHoleRestClient {
    pub url: String,
    pub insecure: bool,
}

impl PiHoleClient for PiHoleRestClient {
    fn summary_raw(&self) -> Result<SummaryRaw, Error> {
        let url = format!("{}/admin/api.php?summaryRaw", self.url);
        let mut client_builder = Client::builder();
        if self.insecure {
            client_builder = client_builder.danger_accept_invalid_certs(true);
        }
        let client = client_builder.build().unwrap();
        client.get(url).send().unwrap().json::<SummaryRaw>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_summary_raw() {
        let client = PiHoleRestClient{
            url: mockito::server_url(), 
            insecure: true
        };

        let _m = mockito::mock("GET", "/admin/api.php?summaryRaw")
            .with_status(200)
            .with_body(r#"
                    {
                        "domains_being_blocked": 20000,
                        "dns_queries_today": 1000,
                        "ads_blocked_today": 500,
                        "ads_percentage_today": 50.0,
                        "unique_domains": 15000,
                        "queries_forwarded": 19500,
                        "queries_cached": 1000,
                        "clients_ever_seen": 42,
                        "unique_clients": 42,
                        "dns_queries_all_types": 42,
                        "privacy_level": 1,
                        "status": "OK",
                        "reply_NODATA": 1000,
                        "reply_NXDOMAIN": 1000,
                        "reply_IP": 1000
                    }
            "#).create();

        let result = client.summary_raw();
        assert!(result.is_ok());
    }
}
