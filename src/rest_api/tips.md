# Tips

'curl --location --request POST '127.0.0.1:3030/v1/evm_nodes' --header 'Content-Type: application/json' --header 'Content-Type: text/plain' --data-raw '{"name": "ethereum-mainnet", "url": "https://rpc.api", "client_version": "geth1.8"}''

## GET 1

curl --location --request GET '127.0.0.1:3030/v1/evm_nodes' --header 'Content-Type: application/json' --header 'Content-Type: text/plain' --data-raw '{"url": "https://rpc.api", "info": {"name": "ethereum-mainnet",  "client_version": "geth1.8"}}'

## POST 1

curl --location --request POST '127.0.0.1:3030/v1/evm_nodes' --header 'Content-Type: application/json' --header 'Content-Type: text/plain' --data-raw '{"url": "rpc_1", "info": {"name": "mainnet",  "client_version": "geth1"}}'


## POST 2

curl --location --request POST '127.0.0.1:3030/v1/evm_nodes' --header 'Content-Type: application/json' --header 'Content-Type: text/plain' --data-raw '{"url": "rpc_2", "info": {"name": "testnet",  "client_version": "geth2"}}'

## UPDATE 1

curl --location --request PUT '127.0.0.1:3030/v1/evm_nodes' --header 'Content-Type: application/json' --header 'Content-Type: text/plain' --data-raw '{"url": "rpc_1", "info": {"name": "testnet2",  "client_version": "geth1"}}'

## DELETE 1

curl --location --request DELETE '127.0.0.1:3030/v1/evm_nodes' --header 'Content-Type: application/json' --header 'Content-Type: text/plain' --data-raw '{"url": "rpc_2", "info": {"name": "testnet2",  "client_version": "geth1"}}'

## GET 3

curl --location --request GET '127.0.0.1:3030/v1/evm_nodes' --header 'Content-Type: application/json' --header 'Content-Type: text/plain' --data-raw '{"url": "rpc_2", "info": {"name": "ethereum-mainnet",  "client_version": "geth1.8"}}'