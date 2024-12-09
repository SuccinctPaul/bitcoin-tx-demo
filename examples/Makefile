help: ## Display this help screen
	@grep -h \
		-E '^[a-zA-Z_0-9-]+:.*?## .*$$' $(MAKEFILE_LIST) | \
		awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'


BITCOIND=bitcoind
BITCOINGUI=bitcoin-qt
BITCOINCLI=bitcoin-cli
B1_FLAGS=
B2_FLAGS=
B1=-datadir=. $(B1_FLAGS)

BLOCKS=1
## Address param
ADDRESS=
## AMOUNT param
AMOUNT=
## ACCOUNT param
ACCOUNT=

CLI=


######### Start/Stop related
start: ## start node1
	$(BITCOIND) $(B1)  -daemon
start-gui: ## Start Node GUI
	$(BITCOINGUI)
stop: ## Stop the node
	$(BITCOINCLI) $(B1)  stop

clean:
	find regtest/* -not -name 'server.*' -delete

run:## support more sub-command, param:CLI
	$(BITCOINCLI)  $(B1)  $(CLI)

######## node status
getinfo: ## getinfo
	$(BITCOINCLI) $(B1) -getinfo
getblockchaininfo: ## getblockchaininfo
	$(BITCOINCLI) $(B1) getblockchaininfo
getmininginfo: ## getmininginfo node1
	$(BITCOINCLI) $(B1) getmininginfo
getblock: ## getblock
	$(BITCOINCLI) $(B1) getblock



######## wallet related
create_wallet: ## create wallet: make create_wallet1 ACCOUNT=wallet1
	$(BITCOINCLI) $(B1) createwallet $(ACCOUNT)

getWalletInfo: ## getwalletinfo, param: ACCOUNT=wallet1
	$(BITCOINCLI) $(B1)  -rpcwallet=$(ACCOUNT) getwalletinfo

listwallets: ## getaddressesbylabel
	$(BITCOINCLI) $(B1)  -rpcwallet=$(ACCOUNT) listaddressgroupings
	#$(BITCOINCLI) $(B1)   listaddressgroupings





####### Address related
getnewaddress: ## getnewaddress, param: ACCOUNT
	$(BITCOINCLI) $(B1)  -rpcwallet=$(ACCOUNT) getnewaddress  '' bech32m

getaddressesbylabel: ## getaddressinfo, param: ACCOUNT=wallet1
	$(BITCOINCLI) $(B1)  -rpcwallet=$(ACCOUNT) getaddressesbylabel ""

listaddressgroupings: ## listaddressgroupings, param: ACCOUNT=wallet1
	$(BITCOINCLI) $(B1)  -rpcwallet=$(ACCOUNT) listaddressgroupings



####### mine/gen block related
generate: ## generate blocks, param: BLOCKS
	$(BITCOINCLI) $(B1) -generate $(BLOCKS)

mine: ## miner for some address, skip 100 block, then we can mine.
	$(BITCOINCLI) $(B1)  generatetoaddress 1 $(ADDRESS)

sendfrom: ## send trx, param: ADDRESS, AMOUNT
	$(BITCOINCLI) $(B1) -rpcwallet=$(ACCOUNT) sendtoaddress $(ADDRESS) $(AMOUNT)


load_wallet: ##
	$(BITCOINCLI) $(B1) loadwallet $(ACCOUNT)

####### transactions related



###### mempool





###### Dummy wallet
WALLET_FROM=benefactor - node
WALLET_RECIEVER=beneficiary

import_benefactor: ## import benefactor
	@$(BITCOINCLI) $(B1)  -rpcwallet=$(WALLET_FROM) importdescriptors '[{ \
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
	@$(BITCOINCLI) $(B1)  -rpcwallet=$(WALLET_RECIEVER) importdescriptors '[ \
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

load_test_wallet: ##
	$(BITCOINCLI) $(B1) loadwallet beneficiary
	$(BITCOINCLI) $(B1) loadwallet benefactor


.PHONY: clippy fmt test