use ethers::{
    types::transaction::eip2718::TypedTransaction,
    utils::{hex, rlp::Rlp},
};

fn main() {
    /* Should resolve to something like this
    {
        "from": "0x19E7E376E7C213B7E7e7e46cc70A5dD086DAff2A",
        "to": "0x000000000000000000000000000000000000dEaD",
        "value": 420,
        "data": "0x",
        "nonce": 69,
        "gasPrice": 1_337,
        "gas": 1_234_567
    }
    */
    let tx_hex = hex::decode("0xf864458205398312d68794000000000000000000000000000000000000dead8201a4801ba05ba31ae2eae3287d21fb1e09f3f65b0455c59e8c3a2d8164ed4502b39bd12ffca0061ffa04123d14529885914c69ec6f939bcde680f7c03fe4f1493daedebabd97").unwrap();
    // https://github.com/gakonst/ethers-rs/issues/561
    // https://github.com/gakonst/ethers-rs/pull/805
    // https://github.com/gakonst/ethers-rs/pull/805/files#diff-c57e9a06ef7b5f55c5498a79e30dfd71f1e395b11c58a50d340a6276bf0dec65R395-R406
    let tx_req = TypedTransaction::decode_signed(&Rlp::new(tx_hex.as_slice()));
    println!("{:?}", tx_req);
}
