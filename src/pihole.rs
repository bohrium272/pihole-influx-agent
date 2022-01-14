use reqwest::blocking::Client;
use reqwest::Error;
use crate::summary::SummaryRaw;

pub trait PiHoleClient {
    fn summary_raw(&self) -> Result<SummaryRaw, Error>;
}

pub struct PiHoleRestClient {
    pub hostname: String,
    pub password: String,
    pub https: bool,
    pub insecure: bool,
}

impl PiHoleClient for PiHoleRestClient {
    fn summary_raw(&self) -> Result<SummaryRaw, Error> {
        let protocol = if self.https { "https" } else { "http" };
        let url = format!("{}://{}/admin/api.php?summaryRaw", protocol, self.hostname);
        let mut client_builder = Client::builder();
        if self.insecure {
            client_builder = client_builder.danger_accept_invalid_certs(true);
        }
        let client = client_builder.build().unwrap();
        return client.get(url)
            .send()
            .unwrap()
            .json::<SummaryRaw>();
    }
}
