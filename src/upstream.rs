use reqwest::header::CONTENT_TYPE;
use reqwest::Client;

use crate::error::Result;
use crate::udp_server::Request;

/// Upstream represents a single DNS-over-HTTPS upstream service.
pub struct Upstream<'a> {
    /// An HTTP client to be used for all requests.
    client: &'a Client,
    /// The URL of the upstream service.
    url: &'a str,
}

impl<'a> Upstream<'a> {
    /// Return a new Upstream with the given client and URL.
    pub fn new(client: &'a Client, url: &'a str) -> Upstream<'a> {
        Upstream { client, url }
    }

    /// Send a given request to the upstream service and return the raw response.
    pub async fn send(&self, request: &Request) -> Result<Vec<u8>> {
        let response = self
            .client
            .post(self.url)
            .header(CONTENT_TYPE, "application/dns-message")
            .body(request.body.to_owned())
            .send()
            .await?;

        Ok(response.bytes().await?.to_vec())
    }
}
