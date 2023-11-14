from web3 import Web3
from eth_account import Account

account = Account.from_key('1'*64)
print(account.privateKey)
print(account.address)

tx = {
    "from": account.address,
    "to": "0x000000000000000000000000000000000000dEaD",
    "value": 420,
    "data": "0x",
    "nonce": 69,
    "gasPrice": 1_337,
    "gas": 1_234_567
}

signed_tx = account.sign_transaction(tx)
print(signed_tx.rawTransaction.hex())