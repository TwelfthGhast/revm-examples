use ethers::{
    core::abi::parse_abi,
    providers::{Http, Provider},
    contract::BaseContract,
};
use revm::{
    db::EthersDB,
    primitives::{address, ExecutionResult, Output, TransactTo, U256},
    EVM,
};
use std::sync::Arc;
use tokio;

use std::time::Instant;

#[tokio::main]
async fn main() {
    get_reserves_using_ethersdb().await;
}

async fn get_reserves_using_ethersdb() -> (u128, u128, u32) {
    // create ethers client and wrap it in Arc<M>
    let client = Provider::<Http>::try_from(
        "https://mainnet.infura.io/v3/c60b0bb42f8a4c6481ecd229eddaca27",
    ).unwrap();
    let client = Arc::new(client);

    // ----------------------------------------------------------- //
    //             Storage slots of UniV2Pair contract             //
    // =========================================================== //
    // storage[5] = factory: address                               //
    // storage[6] = token0: address                                //
    // storage[7] = token1: address                                //
    // storage[8] = (res0, res1, ts): (uint112, uint112, uint32)   //
    // storage[9] = price0CumulativeLast: uint256                  //
    // storage[10] = price1CumulativeLast: uint256                 //
    // storage[11] = kLast: uint256                                //
    // =========================================================== //

    // ETH/USDT pair on Uniswap V2
    let pool_address = address!("0d4a11d5EEaaC28EC3F61d100daF4d40471f1852");

    // generate abi for the calldata from the human readable interface
    let abi = BaseContract::from(
        parse_abi(&[
            "function getReserves() external view returns (uint112 reserve0, uint112 reserve1, uint32 blockTimestampLast)",
        ]).unwrap()
    );

    // encode abi into Bytes
    let encoded = abi.encode("getReserves", ()).unwrap();

    // initialize new EthersDB
    let ethersdb = EthersDB::new(Arc::clone(&client), None).unwrap();

    // initialise an empty (default) EVM
    let mut evm = EVM::new();

    // insert pre-built database from above
    evm.database(ethersdb);

    // fill in missing bits of env struct
    // change that to whatever caller you want to be
    evm.env.tx.caller = address!("0000000000000000000000000000000000000000");
    // account you want to transact with
    evm.env.tx.transact_to = TransactTo::Call(pool_address);
    // calldata formed via abigen
    evm.env.tx.data = encoded.0.into();
    // transaction value in wei
    evm.env.tx.value = U256::from(0);

    let tx_start = Instant::now();
    // execute transaction without writing to the DB
    let ref_tx = evm.transact().unwrap();

    let execution = ref_tx.state;
    for (address, account) in execution.iter() {
        println!("Address: {}", address);
        println!("Address Storage:");
        for (key, storage_slot) in account.storage.iter() {
            println!("{} {} {}", key, storage_slot.previous_or_original_value, storage_slot.present_value);
        }
        println!("Address is contract: {}", account.info.code.is_some());
        println!("------------------------------------------------------")
    }
    // select ExecutionResult struct
    let result = ref_tx.result;

    // unpack output call enum into raw bytes
    let value = match result {
        ExecutionResult::Success {
            output: Output::Call(value),
            ..
        } => value,
        result => panic!("Execution failed: {result:?}"),
    };

    // decode bytes to reserves + ts via ethers-rs's abi decode
    let (reserve0, reserve1, ts): (u128, u128, u32) = abi.decode_output("getReserves", value).unwrap();

    // Print emulated getReserves() call output
    println!("Reserve0: {:#?}", reserve0);
    println!("Reserve1: {:#?}", reserve1);
    println!("Timestamp: {:#?}", ts);

    println!("EthersDB: Finished in {:?}", tx_start.elapsed());
    
    (reserve0, reserve1, ts)
}