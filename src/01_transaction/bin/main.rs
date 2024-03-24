use anyhow::Result;
use shared_crypto::intent::Intent;
use std::env;
use std::str::FromStr;
use sui_keys::keystore::{AccountKeystore, FileBasedKeystore};
use sui_sdk::{
    rpc_types::{SuiObjectDataOptions, SuiObjectResponseQuery, SuiTransactionBlockResponseOptions},
    types::{
        base_types::SuiAddress,
        programmable_transaction_builder::ProgrammableTransactionBuilder,
        quorum_driver_types::ExecuteTransactionRequestType,
        transaction::{Transaction, TransactionData},
    },
    SuiClientBuilder,
};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Inital part to pull sender address from sui.keystore - open keystore (just to demostrate this feature)
    let args: Vec<String> = env::args().collect();
    let sui_keystore = &args[1];
    let keystore = FileBasedKeystore::new(&sui_keystore.into())?;

    // Connnection string to SUI devnet
    let sui_local = SuiClientBuilder::default()
        .build("https://fullnode.devnet.sui.io:443")
        .await?;
    // Just in case you will need it for debug
    println!("\nSui local version: {}", sui_local.api_version());

    // SUI sender address pulled from keystore file
    let my_address = keystore.addresses()[0];
    println!("\nSender address: {}", my_address);

    // In sender wallet is only one object 0x1077a301f31aa4351c873c161ae9b052528a2c401a49bc75f4cc466483f8b759 > check README.md with output
    let coins_response = &sui_local
        .read_api()
        .get_owned_objects(
            my_address,
            Some(SuiObjectResponseQuery::new_with_options(
                SuiObjectDataOptions::new().with_type(),
            )),
            None,
            None,
        )
        .await?;
    println!("\nWallet coins: {:?}", coins_response);

    // Find SUI / usable coin for gas
    let coin = coins_response
        .data
        .iter()
        .find(|obj| obj.data.as_ref().unwrap().is_gas_coin())
        .unwrap();
    let coin = coin.data.as_ref().unwrap();
    println!("\nWallet SUI: {:?}", coin);

    // Selected coin object balance check
    let balance = sui_local
        .coin_read_api()
        .get_coins(my_address, None, None, None)
        .await?;
    let coin_balance = balance.data.into_iter().next().unwrap();
    println!("\nSUI balance: {:?}", coin_balance);

    // Receiver address / it could be also parameter etc. this is just simple example :)
    let _rec = "0x8d5fbe4b69445fbe7dd11133b8221e2f1da482c3e751fbee218cc4953e84de8e";

    // Conversion of address to SuiAddress
    match SuiAddress::from_str(_rec) {
        // Happy path
        Ok(recipient) => {
            // Block build with amount 1,000,000,000 = 1 SUI
            let pt = {
                let mut builder = ProgrammableTransactionBuilder::new();
                builder.pay_sui(vec![recipient], vec![100000000])?; //0.1SUI
                builder.finish()
            };

            let gas_budget = 5_000_000;
            let gas_price = sui_local.read_api().get_reference_gas_price().await?;

            // Create the transaction data that will be sent to the network
            let tx_data = TransactionData::new_programmable(
                my_address,
                vec![coin.object_ref()],
                pt,
                gas_budget,
                gas_price,
            );

            // Signing transaction
            let signature =
                keystore.sign_secure(&my_address, &tx_data, Intent::sui_transaction())?;

            // Transaction execution
            print!("\nExecuting the transaction...");
            let transaction_response = sui_local
                .quorum_driver_api()
                .execute_transaction_block(
                    Transaction::from_data(tx_data, vec![signature]),
                    SuiTransactionBlockResponseOptions::full_content(),
                    Some(ExecuteTransactionRequestType::WaitForLocalExecution),
                )
                .await?;
            print!("done!\n\nTransaction information: ");
            println!("{:?}", transaction_response);

            // Double check of the receiver object balance after transaction
            let coins = sui_local
                .coin_read_api()
                .get_coins(recipient, None, None, None)
                .await?;
            println!(
                "\nAfter the transfer, the recipient address {recipient} has {} coins",
                coins.data.len()
            );

            println!("\nAll done!");
        }
        Err(err) => {
            eprintln!("Error parsing SuiAddress: {}", err);
            // Handle the error...
        }
    }
    Ok(())
}
