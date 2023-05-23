mod vantage;
use vantage::{Args, PricesResponse, Price};
#[macro_use]
extern crate prettytable;
use clap::Parser;
use prettytable::Table;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let mut op_table = Table::new();
    op_table.add_row(row!["Id", "Region", "Unit", "Currency", "Amount"]);
    let inputs: Args = Args::parse();
    let args = Args::new(
        inputs.api_token,
        inputs.region,
        inputs.platform,
        inputs.instance_identifier,
    );
    let result = args.get_info().await?;
    let data: PricesResponse = serde_json::from_str(&result).unwrap();

    let filtered_prices: Vec<&Price> = data
        .prices
        .iter()
        .filter(|price| price.region == args.region && price.details.platform == args.platform)
        .collect();

    for price in filtered_prices {
        op_table.add_row(row![
            price.id,
            price.region,
            price.unit,
            price.currency,
            price.amount
        ]);
    }
    op_table.printstd();
    Ok(())
}
