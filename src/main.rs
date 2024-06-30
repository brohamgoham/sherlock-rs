use serde_json::json;
use sherlock_rs::{model, web3};
use model::{JsonRpcResult, Tag};
use web3::Web3;

const BLAST_BASE_URL: &str = "https://blast-mainnet.g.alchemy.com/v2/xxx";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    
    let rpc = Web3::new(BLAST_BASE_URL.to_string());

    let r = rpc.web3_client_version().await?;
    println!("test1");
    println!("{:?}", r);

    let r = rpc.web3_sha3("0x68656c6c6f20776f726c64").await?;
    println!("test2");
    println!("{:?}", r);

    let r = rpc.net_version().await?;
    println!("test3");
    println!("{:?}", r);

   // let r = rpc.net_listening().await?;
  //  println!("test4");
    //println!("{:?}", r);

  //  let r = rpc.net_peer_count().await?;
  //  println!("test5");

    //println!("{:?}", r);

  //  let r = rpc.eth_protocol_version().await?;
  //  println!("{:?}", r);

  //  let r = rpc.eth_syncing().await?;
    //println!("{:?}", r);

   // let r = rpc.eth_coinbase().await?;
   // println!("{:?}", r);

 //   let r = rpc.eth_mining().await?;
 //   println!("{:?}", r);

 //   let r = rpc.eth_hashrate().await?;
  //  println!("{:?}", r);

  //  let r = rpc.eth_gas_price().await?;
  //  println!("{:?}", r);

 //   let r = rpc.eth_accounts().await?;
 //   println!("{:?}", r);

    let r = rpc
        .eth_get_balance(
            "0x846c4dc9f4e2514206ef179eaa0bcfae007e37d2",
            Some(Tag::Latest),
        )
        .await?;
    println!("going to get balance {:?}", r);

    let r = rpc
        .eth_get_storage_at("0x295a70b2de5e3953354a6a8344e616ed314d7251", "0x0", None)
        .await?;
    println!("getting storage {:?}", r);

    let r = rpc
        .eth_get_transaction_count("0x846c4dc9f4e2514206ef179eaa0bcfae007e37d2", None)
        .await?;
    println!("geting tx count {:?}", r);

    match rpc
        .eth_get_block_transaction_count_by_hash(
            "0xe812a49745d691961893d7cfd3902d78d710751bab872f12215ee23f27f3efa9",
        )
        .await
    {
        Ok(r) => {
            println!("eth_get_block_transaction_count_by_hash {:?}", r);
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }

    let rpc =
        Web3::new(BLAST_BASE_URL.to_string());
    let r = rpc.eth_block_number().await?;
    println!("{:?}", r);

    let r = rpc
        .eth_get_transaction_receipt(
            "0x9aba9e042034f025ebbe9fddde27c2c091afe6bb24d1c6f6418abbb3103519ee",
        )
        .await?;
    println!("{:?}", r);

    let r = rpc
        .eth_get_transaction_by_block_hash_and_index(
            "0x5cd7861dde444b29a5d362a015795c291b8a51ec7e6bbb6e31ade8cfc96716b9",
            "0x0",
        )
        .await?;
    println!("{:?}", r);

    let r = rpc
        .eth_get_transaction_by_block_number_and_index("0xdf969d", "0x0")
        .await?;
    println!("{:?}", r);

    let r = rpc
        .eth_get_uncle_by_block_hash_and_index(
            "0x5cd7861dde444b29a5d362a015795c291b8a51ec7e6bbb6e31ade8cfc96716b9",
            "0x0",
        )
        .await?;
    println!("{:?}", r);


    Ok(())
}