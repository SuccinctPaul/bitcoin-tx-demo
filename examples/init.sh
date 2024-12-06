#!/usr/bin/env bash
# Reference https://github.com/rust-bitcoin/rust-bitcoin/blob/master/bitcoin/examples/taproot-psbt.rs
alias bt='bitcoin-cli -regtest'

#2.1) Run `bt -named createwallet wallet_name=benefactor blank=true` to create a blank wallet with the name "benefactor"
bt -named createwallet wallet_name=benefactor blank=true
#2.2) Run `bt -named createwallet wallet_name=beneficiary blank=true` to create a blank wallet with the name "beneficiary"
bt -named createwallet wallet_name=benefactor blank=true
#2.3) Create the two aliases:
alias bt-benefactor='bitcoin-cli -regtest -rpcwallet=benefactor'

#2.4) Import the example descriptors:
bt-benefactor importdescriptors '[ \
   { "desc": "tr(tprv8ZgxMBicQKsPd4arFr7sKjSnKFDVMR2JHw9Y8L9nXN4kiok4u28LpHijEudH3mMYoL4pM5UL9Bgdz2M4Cy8EzfErmU9m86ZTw6hCzvFeTg7/86\'/1\'/0\'/1/*) \ #jzyeered", "active": true, "timestamp": "now", "internal": true },
   { "desc": "tr(tprv8ZgxMBicQKsPd4arFr7sKjSnKFDVMR2JHw9Y8L9nXN4kiok4u28LpHijEudH3mMYoL4pM5UL9Bgdz2M4Cy8EzfErmU9m86ZTw6hCzvFeTg7/86\'/1\'/0\'/0/*)#rkpcykf4", "active": true, "timestamp": "now" } \
]'

bt-beneficiary importdescriptors '[
   {
     "desc": "tr(tprv8ZgxMBicQKsPe72C5c3cugP8b7AzEuNjP4NSC17Dkpqk5kaAmsL6FHwPsVxPpURVqbNwdLAbNqi8Cvdq6nycDwYdKHDjDRYcsMzfshimAUq/86\'/1\'/0\'/1/*)#w4ehwx46",
     "active": true, "timestamp": "now", "internal": true
  },
  {
     "desc": "tr(tprv8ZgxMBicQKsPe72C5c3cugP8b7AzEuNjP4NSC17Dkpqk5kaAmsL6FHwPsVxPpURVqbNwdLAbNqi8Cvdq6nycDwYdKHDjDRYcsMzfshimAUq/86\'/1\'/0\'/0/*)#lpuknn9z",
     "active": true, "timestamp": "now"
  }
]'


bt-beneficiary importdescriptors '[
    {
      "desc": "tr(tprv8ZgxMBicQKsPe72C5c3cugP8b7AzEuNjP4NSC17Dkpqk5kaAmsL6FHwPsVxPpURVqbNwdLAbNqi8Cvdq6nycDwYdKHDjDRYcsMzfshimAUq/86\'/1\'/0\'/1/*)#w4ehwx46",
      "active": true, "timestamp": "now", "internal": true
    },
    {
      "desc": "tr(tprv8ZgxMBicQKsPe72C5c3cugP8b7AzEuNjP4NSC17Dkpqk5kaAmsL6FHwPsVxPpURVqbNwdLAbNqi8Cvdq6nycDwYdKHDjDRYcsMzfshimAUq/86\'/1\'/0\'/0/*)#lpuknn9z",
      "active": true, "timestamp": "now"
    }
]'


# generate benefactor address
bt-benefactor getnewaddress '' bech32m
bt-benefactor getnewaddress '' bech32m
bt-benefactor getnewaddress '' bech32m
bt-benefactor getnewaddress '' bech32m