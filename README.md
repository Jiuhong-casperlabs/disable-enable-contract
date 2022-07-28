## prepare
```
cd disable-enable-contract
make prepare
make build-contract
```
---
## Work Flow
### deploy contract to blockchain
```
casper-client put-deploy --chain-name mynetwork \
--node-address http://16.162.124.124:7777 \
--secret-key /home/jh/keys/test99/secret_key.pem \
--session-path /home/jh/mynetwork/contracts/helloworld/contract/target/wasm32-unknown-unknown/release/contract.wasm \
--payment-amount 30000000000
```

=> success

contract_hash: (yours are different)
hash-28db4e0bdda0841787adef1a37bb5e4de4d9a73020ce5210976e7ee9521d6cb3
contract_package_hash:
hash-2b626bfebe3636e090f0fc63d6124f5399eb5078f2f49160f974c6f4d94ec757


### call entry point hello_world and it works fine
```
casper-client put-deploy --chain-name mynetwork \
--node-address http://16.162.124.124:7777 \
--secret-key /home/jh/keys/test99/secret_key.pem \
--session-hash hash-28db4e0bdda0841787adef1a37bb5e4de4d9a73020ce5210976e7ee9521d6cb3 \
--session-entry-point hello_world \
--payment-amount 10000000000
```

=> success


### disable contract=(non-admin) and it will return error
```
casper-client put-deploy --chain-name mynetwork \
--node-address http://16.162.124.124:7777 \
--secret-key /home/jh/keys/test99/secret_key.pem \
--session-arg "contract_package_hash:account_hash='account-hash-2b626bfebe3636e090f0fc63d6124f5399eb5078f2f49160f974c6f4d94ec757'" \
--session-arg "contract_hash:account_hash='account-hash-28db4e0bdda0841787adef1a37bb5e4de4d9a73020ce5210976e7ee9521d6cb3'" \
--session-path ~/casper-node/target/wasm32-unknown-unknown/release/disable_contract.wasm \
--payment-amount 3000000000
```

=>
"error_message": "Forged reference: URef(ffdfd3bf838857518eef0e0010aeac509bf914d67f6dc440601396a2be42be47, READ_ADD_WRITE)",


### disable contract=(admin)==
```
casper-client put-deploy --chain-name mynetwork \
--node-address http://16.162.124.124:7777 \
--secret-key /home/jh/keys/test1/secret_key.pem \
--session-arg "contract_package_hash:account_hash='account-hash-2b626bfebe3636e090f0fc63d6124f5399eb5078f2f49160f974c6f4d94ec757'" \
--session-arg "contract_hash:account_hash='account-hash-28db4e0bdda0841787adef1a37bb5e4de4d9a73020ce5210976e7ee9521d6cb3'" \
--session-path ~/casper-node/target/wasm32-unknown-unknown/release/disable_contract.wasm \
--payment-amount 3000000000
```

=> success


### call entry point hello_worldcall again =>
```
casper-client put-deploy --chain-name mynetwork \
--node-address http://16.162.124.124:7777 \
--secret-key /home/jh/keys/test99/secret_key.pem \
--session-hash hash-28db4e0bdda0841787adef1a37bb5e4de4d9a73020ce5210976e7ee9521d6cb3 \
--session-entry-point hello_world \
--payment-amount 10000000000
```

=> "error_message": "Contract is disabled"


###  enable contract (non-admin) and it won't work
```
casper-client put-deploy --chain-name mynetwork \
--node-address http://16.162.124.124:7777 \
--secret-key /home/jh/keys/test99/secret_key.pem \
--session-arg "contract_package_hash:account_hash='account-hash-2b626bfebe3636e090f0fc63d6124f5399eb5078f2f49160f974c6f4d94ec757'" \
--session-arg "contract_hash:account_hash='account-hash-28db4e0bdda0841787adef1a37bb5e4de4d9a73020ce5210976e7ee9521d6cb3'" \
--session-path ~/casper-node/target/wasm32-unknown-unknown/release/enable_contract.wasm \
--payment-amount 3000000000
```

=> "error_message": "Forged reference: URef(ffdfd3bf838857518eef0e0010aeac509bf914d67f6dc440601396a2be42be47, READ_ADD_WRITE)",


### enable contract (admin)
```
casper-client put-deploy --chain-name mynetwork \
--node-address http://16.162.124.124:7777 \
--secret-key /home/jh/keys/test1/secret_key.pem \
--session-arg "contract_package_hash:account_hash='account-hash-2b626bfebe3636e090f0fc63d6124f5399eb5078f2f49160f974c6f4d94ec757'" \
--session-arg "contract_hash:account_hash='account-hash-28db4e0bdda0841787adef1a37bb5e4de4d9a73020ce5210976e7ee9521d6cb3'" \
--session-path ~/casper-node/target/wasm32-unknown-unknown/release/enable_contract.wasm \
--payment-amount 3000000000
```

=> success


### call entry point hello_worldcall again =>
```
casper-client put-deploy --chain-name mynetwork \
--node-address http://16.162.124.124:7777 \
--secret-key /home/jh/keys/test99/secret_key.pem \
--session-hash hash-28db4e0bdda0841787adef1a37bb5e4de4d9a73020ce5210976e7ee9521d6cb3 \
--session-entry-point hello_world \
--payment-amount 10000000000
```

=> success
