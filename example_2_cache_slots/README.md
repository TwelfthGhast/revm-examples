## Me trying to figure out how to determine what storage slots a tx will need

For an arbitrary transaction, it's probably easier to see what storage slots have been touched in a simulation rather than reading through the code, esp if some contracts are unverified. Ideally you would want all relevant data to be cached before trying to simulate a bundle - this ends up being very easy if you generally interact with the same types of contracts.

Turns out, it's quite easy to do this in revm. Simply iterate through the `state` field of the tx rather than just the `result` field to have a list of storage slots accessed as well as what addresses are accessed and if they have `code` associated with them.

## Output

Using the same base code as `example_1`:
```
Address: 0x0000000000000000000000000000000000000000
Address Storage:
Address is contract: true
------------------------------------------------------
Address: 0x0d4a11d5EEaaC28EC3F61d100daF4d40471f1852
Address Storage:
8 45830772568004867244206983513429878215966152467857592223382191800318541270835 45830772568004867244206983513429878215966152467857592223382191800318541270835
Address is contract: true
------------------------------------------------------
Reserve0: 17652263111663375191859
Reserve1: 36140124230746
Timestamp: 1699957835
EthersDB: Finished in 1.458627615s
```

This matches exactly slot8 of the UniswapV2 Pair contract which the revm examples have mentioned.