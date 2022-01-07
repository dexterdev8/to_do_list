## Run tests:

`cargo test`

## Build your smart contract:

`cargo build --target wasm32-unknown-unknown --release`

## Deploy your smart contract to as a subaccount of dexterdev8.testnet:

`near deploy --accountId to-do-tasks.dexterdev8.testnet --wasmFile target/wasm32-unknown-unknown/release/to_do_contract.wasm`

## Delete the subaccount so you can deploy again your smart contract:

`near delete to-do-tasks.dexterdev8.testnet dexterdev8.testnet`
