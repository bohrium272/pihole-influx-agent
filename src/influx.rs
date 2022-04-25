use reqwest::blocking::{Client, Response};
use reqwest::Error;

pub trait InfluxMetric {
    fn influx_metric(self) -> String;
}

pub struct InfluxClient {
    pub url: String,
    pub token: String,
    pub org_id: String,
    pub insecure: bool,
}

impl InfluxClient {
    pub fn write<T: InfluxMetric>(&self, bucket: String, metric: T) -> Result<Response, Error> {
        let url = format!(
            "{}/api/v2/write?bucket={}&orgID={}",
            self.url, bucket, self.org_id
        );
        let mut client_builder = Client::builder();
        if self.insecure {
            client_builder = client_builder.danger_accept_invalid_certs(true);
        }
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "Authorization",
            format!("Token {}", self.token).parse().unwrap(),
        );
        let client = client_builder.build().unwrap();
        
        client
            .post(url)
            .headers(headers)
            .body(metric.influx_metric())
            .send()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestMetric;

    impl InfluxMetric for TestMetric {
        fn influx_metric(self) -> String {
            "test-metric".to_string()
        }
    }

    #[test]
    fn test_write() {
        let client = InfluxClient{
            url: mockito::server_url(),
            token: "<token>".to_string(),
            org_id: "org_id".to_string(),
            insecure: true
        };

        let _m = mockito::mock("POST", "/write?bucket=bucket&orgID=org_id")
            .with_status(201)
            .with_header("Authorization", "Token <token>")
            .with_body("metric")
            .create();

        let result = client.write::<TestMetric>("bucket".to_string(), TestMetric);

        assert!(result.is_ok());
    }
}
