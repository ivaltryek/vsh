use serde::Deserialize;
use clap::Parser;
use colored::Colorize;
use reqwest::header::ACCEPT;
const VANTAGE_API_URI: &str = "https://api.vantage.sh/v1/products";
#[derive(Debug, Deserialize)]
pub struct Price {
   pub id: String,
   pub unit: String,
   pub region: String,
   // rate_type: String,
   pub currency: String,
   pub amount: f64,
   pub details: Details,
}

#[derive(Debug, Deserialize)]
pub struct Details {
    pub platform: String,
    // lifecycle: String,
}

#[derive(Debug, Deserialize)]
pub struct PricesResponse {
    // links: Links,
   pub prices: Vec<Price>,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    // Vantage API Token to access APIs.
   pub api_token: String,

    #[arg(short, long)]
    // AWS Region ->  ap-south-1, us-east-1
   pub region: String,

    #[arg(short, long)]
    // OS Platform -> windows, linux, suse, rhel
    pub platform: String,

    #[arg(short, long)]
    // Instance identifier -> m5a.large, m6a.large, t3.medium etc
    pub instance_identifier: String,
}

impl Args {
    pub fn new(
        api_token: String,
        region: String,
        platform: String,
        instance_identifier: String,
    ) -> Self {
        Args {
            api_token,
            region,
            platform,
            instance_identifier,
        }
    }

    pub async fn get_info(&self) -> Result<String, reqwest::Error> {
        let format_instance_identifier = &self.instance_identifier.replace(".", "_");
        println!(
            "{}: {}",
            "Fetching details for".yellow(),
            format_instance_identifier.green()
        );
        let uri_builder = format!(
            "{}/aws-ec2-{}/prices",
            VANTAGE_API_URI, format_instance_identifier
        );
        let client = reqwest::Client::new();
        let resp = client
            .get(uri_builder)
            .header(ACCEPT, "application/json")
            .bearer_auth(&self.api_token)
            .send()
            .await?
            .text()
            .await?;
        Ok(resp)
    }
}
