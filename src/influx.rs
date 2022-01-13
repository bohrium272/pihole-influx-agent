use reqwest::Error;
use reqwest::blocking::{Client, Response};

pub trait InfluxMetric {
    fn influx_metric(self) -> String;
}

pub struct InfluxClient {
    pub hostname: String,
    pub token: String,
    pub org_id: String,
    pub https: bool,
    pub insecure: bool
}

impl InfluxClient {
    pub fn write<T: InfluxMetric>(&self, bucket: String, metric: T) -> Result<Response, Error> {
        let protocol = if self.https { "https" } else { "http" };
        let url = format!("{}://{}/api/v2/write?bucket={}&orgID={}", protocol, self.hostname, bucket, self.org_id);
        let mut client_builder = Client::builder();
        if self.insecure {
            client_builder = client_builder.danger_accept_invalid_certs(true);
        }
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Authorization", format!("Token {}", self.token).parse().unwrap());
        let client = client_builder.build().unwrap();
        return client.post(url)
            .headers(headers)
            .body(metric.influx_metric())
            .send()
    }
}
