use anyhow::{Context, Result};
use clap::Parser;
use futures::{ stream::StreamExt};

use std::{
    env, collections::{HashMap, HashSet},
    time::{ Instant, SystemTime, UNIX_EPOCH},
    // fs::{self, File}, path::Path,
    fs::{OpenOptions},
    fs,
    sync::Arc,
    io::{self,BufRead,Write,stdin},
    net::{SocketAddr, IpAddr, Ipv4Addr},
    os::unix::net::UnixStream,
    str::FromStr
};
use tokio::{
    time::{sleep, Duration},
    sync::Mutex,
    sync::RwLock
};


use bincode;
use hex;

use serde_json::json;
use serde_json::Value;
use serde::{Serialize, Deserialize};

use solana_client::{
    rpc_client::RpcClient,
    tpu_client::{TpuClient, TpuClientConfig},
    rpc_response::RpcContactInfo
};
use solana_sdk::{
    bs58,
    signature::{Keypair,Signature,Signer},
    pubkey::Pubkey,
    commitment_config::CommitmentConfig,
    transaction::VersionedTransaction
};
use solana_transaction_status::UiTransactionEncoding;

use spl_associated_token_account::{get_associated_token_address, get_associated_token_address_with_program_id};
use spl_token_2022::{id as token_2022_program_id};

use yellowstone_grpc_client::GeyserGrpcClient;
use yellowstone_grpc_proto::{
    prelude::{
        subscribe_update::UpdateOneof, CommitmentLevel, SubscribeRequest,
        SubscribeRequestFilterSlots, SubscribeRequestPing, SubscribeUpdatePong,
        SubscribeUpdateSlot,SubscribeRequestFilterTransactions,SubscribeRequestFilterEntry,
        SubscribeRequestFilterBlocksMeta, SubscribeRequestFilterBlocks, SubscribeRequestFilterAccounts,
        SubscribeUpdateTransactionInfo
    },
    convert_from
};



///////////////////////////////////////////////////////////////
#[tokio::main]
async fn main() -> Result<()> {
    let solana_mint_address="So11111111111111111111111111111111111111112";

    let system_program="11111111111111111111111111111111";

    let pumpfun_program="6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P";
    let pumpswap_program="pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA";
    let raydium_amm_program="675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8";
    let raydium_cpmm_program="CPMMoo8L3F4NbTegBCKVNunggL7H1ZpdTHKxQB5qKP1C";
    let raydium_cpmm_authority="GpMZbSM2GgvTKHJirzeGfMFoaZ8UR2X7F4v8vHTvxFbL";
    let raydium_clmm_program="CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK";
    let meteora_dlmm_program="LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo";
    let meteora_dyn_program="dbcij3LWUppWqq96dh6gJWwBifmcGfLSB5D4DuSMaqN";
    let raydium_launchpad_program="LanMV9sAd7wArD4vJFi2qDdfnVhFxYSUg6eADduJ3uj";
    let raydium_launchpad_migrator_prgram="RAYpQbFNq9i3mu6cKpTKKRwwHFDeK5AuZz8xvxUrCgw";

    let genius="BabgvVhpSxSYVxsZN8m8RxzkQPasyKrju5tyTXX9SQaR";

    // let lines = read_lines_until_double_enter();

    // let mut unix_stream= UnixStream::connect("/tmp/quic_send_socket")?;

    // let mut all_tokens=HashMap::new();

    // let crypto_provider= crypto_default_provider();
    // crypto_provider.install_default();
    
    
    dotenv::dotenv().ok();
    // let http_client=Client::new();

    // let http_client_escape=http_client.clone();


    //Create web3 connection
    let rpc_url = env::var("RPC_API").unwrap();
    let commitment = CommitmentConfig::processed();
    let rpc_client = RpcClient::new_with_commitment(rpc_url.to_string(),commitment);

    let wallet="DdPv8fFjnkE3mLMsbFcH5aRop4ES3SE7VEZ5Ntz7MVVA";

    let balance = rpc_client.get_balance(Pubkey::from_str_const(wallet)).unwrap();
    println!("{}", balance);

    Ok(())
}



