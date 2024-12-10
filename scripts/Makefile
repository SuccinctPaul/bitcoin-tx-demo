help: ## Display this help screen
	@grep -h \
		-E '^[a-zA-Z_0-9-]+:.*?## .*$$' $(MAKEFILE_LIST) | \
		awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

BITCOINCLI=bitcoin-cli
B1=-regtest

######## node status
getinfo: ## getinfo
	$(BITCOINCLI) $(B1) -getinfo
getblockchaininfo: ## getblockchaininfo
	$(BITCOINCLI) $(B1) getblockchaininfo
getmininginfo: ## getmininginfo node1
	$(BITCOINCLI) $(B1) getmininginfo
getblock: ## getblock
	$(BITCOINCLI) $(B1) getblock

###### Dummy wallet
create_benefactor: ## import benefactor
	@$(BITCOINCLI) $(B1) -named createwallet wallet_name=benefactor blank=true
create_beneficiary: ## import benefactor
	@$(BITCOINCLI) $(B1) -named createwallet wallet_name=beneficiary blank=true

import_benefactor: ## import benefactor
	@$(BITCOINCLI) $(B1)  -rpcwallet=benefactor importdescriptors '[{ \
                                                    "desc": "tr(tprv8ZgxMBicQKsPd4arFr7sKjSnKFDVMR2JHw9Y8L9nXN4kiok4u28LpHijEudH3mMYoL4pM5UL9Bgdz2M4Cy8EzfErmU9m86ZTw6hCzvFeTg7/86'\''/1'\''/0'\''/1/*)#jzyeered", \
                                                    "active": true, \
                                                    "timestamp": "now", \
                                                    "internal": true \
                                                  }, { \
                                                    "desc": "tr(tprv8ZgxMBicQKsPd4arFr7sKjSnKFDVMR2JHw9Y8L9nXN4kiok4u28LpHijEudH3mMYoL4pM5UL9Bgdz2M4Cy8EzfErmU9m86ZTw6hCzvFeTg7/86'\''/1'\''/0'\''/0/*)#rkpcykf4", \
                                                    "active": true, \
                                                    "timestamp": "now" \
                                                }]'

import_beneficiary: ## import beneficiary
	@$(BITCOINCLI) $(B1)  -rpcwallet=beneficiary importdescriptors '[ \
                                                    { \
                                                        "desc": "tr(tprv8ZgxMBicQKsPe72C5c3cugP8b7AzEuNjP4NSC17Dkpqk5kaAmsL6FHwPsVxPpURVqbNwdLAbNqi8Cvdq6nycDwYdKHDjDRYcsMzfshimAUq/86'\''/1'\''/0'\''/1/*)#w4ehwx46", \
                                                        "active": true, \
                                                        "timestamp": "now", \
                                                        "internal": true \
                                                    }, \
                                                    { \
                                                        "desc": "tr(tprv8ZgxMBicQKsPe72C5c3cugP8b7AzEuNjP4NSC17Dkpqk5kaAmsL6FHwPsVxPpURVqbNwdLAbNqi8Cvdq6nycDwYdKHDjDRYcsMzfshimAUq/86'\''/1'\''/0'\''/0/*)#lpuknn9z", \
                                                        "active": true, \
                                                        "timestamp": "now" \
                                                    } \
                                                ]'

load_wallet: ## load wallet
	$(BITCOINCLI) $(B1) loadwallet beneficiary
	$(BITCOINCLI) $(B1) loadwallet benefactor

####### Address related
getnewaddress: ## getnewaddress, param: ACCOUNT
	$(BITCOINCLI) $(B1)  -rpcwallet=beneficiary getnewaddress  '' bech32m
	$(BITCOINCLI) $(B1)  -rpcwallet=benefactor getnewaddress  '' bech32m

getaddressesbylabel_beneficiary: ## getaddressinfo, param: ACCOUNT=wallet1
	$(BITCOINCLI) $(B1)  -rpcwallet=beneficiary getaddressesbylabel ""
getaddressesbylabel_benefactor: ## getaddressinfo, param: ACCOUNT=wallet1
	$(BITCOINCLI) $(B1)  -rpcwallet=benefactor getaddressesbylabel ""

listaddressgroupings_beneficiary: ## listaddressgroupings, param: ACCOUNT=wallet1
	$(BITCOINCLI) $(B1)  -rpcwallet=beneficiary listaddressgroupings
listaddressgroupings_benefactor: ## listaddressgroupings, param: ACCOUNT=wallet1
	$(BITCOINCLI) $(B1)  -rpcwallet=benefactor listaddressgroupings


.PHONY: clippy fmt test