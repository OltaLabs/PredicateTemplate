use fuels::prelude::*;
use std::str::FromStr;

use fuels::crypto::SecretKey;

#[tokio::main]
async fn main() -> Result<()> {
    // Setup provider
    let provider = Provider::connect(
        std::env::var("FUEL_CORE_API").unwrap_or(String::from("http://127.0.0.1:4000")),
    )
    .await?;
    // Create a predicate instance
    let mut predicate = Predicate::load_from(&std::env::var("PREDICATE_BINARY_PATH").unwrap())?;
    predicate.set_provider(provider.clone());

    // Setup wallets
    let wallet = WalletUnlocked::new_from_private_key(
        SecretKey::from_str("de97d8624a438121b86a1956544bd72ed68cd69f2c99555b08b1e8c51ffd511c")?,
        Some(provider.clone()),
    );

    let receiver_wallet = WalletUnlocked::new_random(Some(provider.clone()));

    // Fund the wallet with some test coins
    let asset_id =
        AssetId::from_str("f8f8b6283d7fa5b672b530cbb84fcccb4ff8dc40f8176ef4544ddb1f1952ad07")?;

    // Transfer some funds to the predicate address
    let predicate_address = predicate.address();
    wallet
        .transfer(predicate_address, 500_000, asset_id, TxPolicies::default())
        .await?;

    // Create a transaction to transfer funds from the predicate to the receiver
    let amount_to_transfer = 500_000;
    let mut tb = ScriptTransactionBuilder::prepare_transfer(
        predicate
            .get_asset_inputs_for_amount(asset_id, amount_to_transfer, None)
            .await?,
        predicate.get_asset_outputs_for_amount(
            receiver_wallet.address(),
            asset_id,
            amount_to_transfer,
        ),
        TxPolicies::default(),
    );

    // Sign the transaction with the predicate
    tb.add_signer(wallet.clone())?;

    // Build and send the transaction
    let tx = tb.build(provider.clone()).await?;
    provider.send_transaction_and_await_commit(tx).await?;

    // Check the receiver's balance
    let receiver_balance = receiver_wallet.get_asset_balance(&asset_id).await?;
    println!("Receiver's balance: {}", receiver_balance);

    Ok(())
}
