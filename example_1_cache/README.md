## A basic example of trying to simply the only example present in REVM

https://github.com/bluealloy/revm/blob/310cabaa0a1306687dd4a8218a74c27fb338bbc8/examples/fork_ref_transact.rs

One of the biggest questions I had when browsing this was - how do we know what storage slots we need for an arbitrary transaction? Surely we don't need the whole network state if we don't know what the transaction will do? Foundry/anvil has found a way around this, so surely there's a way to execute a transaction without explicitly storing storage slots.

Turns out if you just pass in EthersDB instead of using CacheDB, it'll grab the necessary data as needed. CacheDB won't work as it'll complain about missing data. Obviously though, prefilling CacheDB and simulating based off cached data is a lot faster.

## Benchmarks

Based off just 1 run:
```
Reserve0: 17659247844643847435491
Reserve1: 36125698871014
Timestamp: 1699956791
EthersDB: Finished in 1.450607503s

Reserve0: 17659247844643847435491
Reserve1: 36125698871014
Timestamp: 1699956791
CacheDB: Finished in 712.744µs
```