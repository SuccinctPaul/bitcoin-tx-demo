use anyhow::{anyhow, bail};
use bitcoin::{Address, Txid};
use reqwest::{Client, Error, Response};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;

const ESPLORA_FAUCET_URL: &str = "https://faucet.mutinynet.com";

#[derive(Debug, Clone)]
pub struct FaucetClient {
    url: String,
    client: Client,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
struct FaucetResponse {
    txid: Txid,
    // address: Address,
    address: String,
}

impl FaucetClient {
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
            client: Client::builder().build().unwrap(),
        }
    }

    pub async fn claim_tokens(
        &self,
        adddr: &str,
        amount: u32,
    ) -> anyhow::Result<FaucetResponse, Error> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(reqwest::header::ACCEPT, "*/*".parse().unwrap());
        headers.insert(reqwest::header::CONNECTION, "keep-alive".parse().unwrap());
        headers.insert(
            reqwest::header::CONTENT_TYPE,
            "application/json".parse().unwrap(),
        );
        headers.insert(reqwest::header::ORIGIN, ESPLORA_FAUCET_URL.parse().unwrap());
        headers.insert(
            reqwest::header::REFERER,
            ESPLORA_FAUCET_URL.parse().unwrap(),
        );

        let data = json!({
            "sats": amount,
            "address": adddr
        });

        let resp = self
            .client
            .post(&format!("{}/api/onchain", ESPLORA_FAUCET_URL))
            .headers(headers)
            .json(&data)
            .send()
            .await?;

        if resp.status().is_server_error() || resp.status().is_client_error() {
            bail!(format!(
                "HttpResponse: {}, {}",
                resp.status().as_u16(),
                resp.text().await?
            ));
        } else {
            Ok(resp.json().await?)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use bitcoin::hashes::sha1::Hash;
    use reqwest::header::HeaderMap;
    use reqwest::Error;
    use std::collections::HashMap;
    use std::time::Instant;

    #[tokio::test]
    async fn test_faucet_request() -> Result<(), Error> {
        use reqwest::Client;
        use serde_json::json;

        let client = Client::new();

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(reqwest::header::ACCEPT, "*/*".parse().unwrap());
        headers.insert(reqwest::header::CONNECTION, "keep-alive".parse().unwrap());
        headers.insert(
            reqwest::header::CONTENT_TYPE,
            "application/json".parse().unwrap(),
        );
        headers.insert(reqwest::header::ORIGIN, ESPLORA_FAUCET_URL.parse().unwrap());
        headers.insert(
            reqwest::header::REFERER,
            ESPLORA_FAUCET_URL.parse().unwrap(),
        );

        let data = json!({
            "sats": 100000,
            "address": "tb1ql9mjwcp9swms3hm6kyvp832myv4ujmqcpmn7az"
        });

        let response = client
            .post(&format!("{}/api/onchain", ESPLORA_FAUCET_URL))
            .headers(headers)
            .json(&data)
            .send()
            .await?;

        println!("Status: {}", response.status());
        println!("Headers:\n{:?}", response.headers());

        let body = response.text().await?;
        println!("Body:\n{}", body);

        Ok(())
    }

    #[tokio::test]
    async fn test_claim_token_from_faucet() {
        let addr = "tb1ql9mjwcp9swms3hm6kyvp832myv4ujmqcpmn7az";

        let faucet = FaucetClient::new(ESPLORA_FAUCET_URL);

        let resp = faucet.claim_tokens(addr, 10000).await.unwrap();

        println!("response: {:?}", resp);
    }
}
