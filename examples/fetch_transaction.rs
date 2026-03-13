//! Fetches a transaction and its receipt from a zkSync Era node.
//!
//! Demonstrates using `RootProvider<Zksync>` to query historical transactions
//! that may omit fields like `accessList`—a response shape that would fail
//! under generic Ethereum typing.
//!
//! # Usage
//!
//! ```sh
//! ZKSYNC_RPC_URL=https://mainnet.era.zksync.io \
//!   cargo run --example fetch_transaction -- <TX_HASH>
//! ```

use alloy::{network::ReceiptResponse as _, primitives::B256, providers::Provider};
use alloy_zksync::provider::zksync_provider;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let rpc_url = std::env::var("ZKSYNC_RPC_URL")
        .unwrap_or_else(|_| "https://mainnet.era.zksync.io".to_string())
        .parse()?;

    let tx_hash: B256 = std::env::args()
        .nth(1)
        .unwrap_or_else(|| {
            // Default: a known type-2 tx on zkSync mainnet that omits `accessList`.
            "0x09d047b22ceb10d30bd1a36969e45eb9f63b6d01f16439f4fd0b9f0114177cff".to_string()
        })
        .parse()?;

    let provider = zksync_provider().connect_http(rpc_url);

    // Fetch the transaction. On zkSync, type-2 transactions may omit `accessList`;
    // the native Zksync network type handles this transparently.
    let tx = provider
        .get_transaction_by_hash(tx_hash)
        .await?
        .expect("transaction not found");
    println!("Transaction: {tx:#?}");

    // Fetch the receipt.
    let receipt = provider
        .get_transaction_receipt(tx_hash)
        .await?
        .expect("receipt not found");
    println!("\nReceipt status: {:?}", receipt.status());
    println!("Block number:   {:?}", receipt.block_number());

    Ok(())
}
