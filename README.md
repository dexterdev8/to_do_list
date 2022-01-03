## Timeline:

December 30 2021: Setup Rust env, Near-cli and testnet account

December 31 2021: Research on Rust and the near-sdk

January 01 2022: Having some fun :)

January 03 2022: Implementing the Smart Contract + unit tests and playing with the calls

## To sum up:

I really enjoied playing aroung with the near-sdk and doing my research on what are the best practices. Of course there is plenty of room for improvement but I am happy with the result so far. I have the feeling that after a review from you guys I can do some optimizations on the code.

Small concern from my side: the events.
I followed an example form the Near doc and by now, the result looks fine in the block explorer but I have the feeling that it can be done in a better way. I hope you can give me some detailed feedback regarding this.

## Links to block explorer:

Contract/Account:
https://explorer.testnet.near.org/accounts/dexterdev18.testnet

call: new
https://explorer.testnet.near.org/transactions/Hog1TUryhEWMtdv2heGNP7BAW8NVUgeeCgez3cX4pa5S

call: create_task
https://explorer.testnet.near.org/transactions/4F34Sg2tWHXCgE49GrHk6A8uCvNwQHXUmjYRkRmXBTUd

call: toggle_completed existent id
https://explorer.testnet.near.org/transactions/7RZFDCgfHGJXxTopc1w4yKZGw8SB7qgfPAnYTwvfKwPD

call: toggle_completed wrong id
https://explorer.testnet.near.org/transactions/ExwDPVb6hUFR2Crj8XdqyLm8uGHeXcMAwezHKKVAXkHY

## Reference links:

https://doc.rust-lang.org/nightly/core/option/enum.Option.html

https://docs.rs/near-sdk/latest/near_sdk/collections/struct.LookupMap.html

https://github.com/near/near-sdk-rs/tree/master/examples/status-message

https://www.near-sdk.io/contract-structure/collections#storage-space

https://www.near-sdk.io/contract-structure/near-bindgen

https://docs.near.org/docs/develop/contracts/rust/near-sdk-rs

https://docs.near.org/docs/tutorials/contracts/nfts/events

