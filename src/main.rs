use dotenv::dotenv;
use hex;
use web3::futures::TryStreamExt;
use web3::types::TransactionId;

#[tokio::main]
async fn main() -> web3::Result {
    dotenv().ok();
    let alchemy_api_key = dotenv::var("ALCHEMY_API_KEY").expect("ALCHEMY_API_KEY must be set");
    let web3 = web3::Web3::new(web3::transports::WebSocket::new(&alchemy_api_key).await?);

    let mut pending_transactions = web3
        .eth_subscribe()
        .subscribe_new_pending_transactions()
        .await?;

    //filter transaction based on address
    let address = "0x31c8eacbffdd875c74b94b077895bd78cf1e64a3";

    while let Some(pending_transaction_hash) = pending_transactions.try_next().await? {
        let transaction = web3
            .eth()
            .transaction(TransactionId::from(pending_transaction_hash))
            .await?;
        if let Some(transaction) = transaction {
            //filter transaction based on address and method hash
            if transaction.to == Some(address.parse().unwrap()) {
                //decode input data bytes to hex
                let tx_clone = transaction.clone();

                let input_data = transaction.input.0;
                let input_data_hex = hex::encode(input_data);
                println!("tx input hex: {:?}", input_data_hex);

                //decode using abi

                if input_data_hex.starts_with("a9059cbb") {
                    let raw_amount = input_data_hex[74..].to_string();
                    println!("Raw Amount cutted hex: {:?}", raw_amount);
                    //decode raw amount to u256
                    let raw_amount = hex::decode(raw_amount).unwrap();
                    //convert to u256
                    let raw_amount = web3::types::U256::from_big_endian(&raw_amount);

                    println!("//---------------------------------------//");
                    println!("Raw Amount: {:?}", raw_amount);
                    println!("Transaction: {:?}", tx_clone);
                    println!("//---------------------------------------//");
                    println!("Transfer in pending towards RAD token contract");
                }
            }
        }
    }

    Ok(())
}
