## Timeline:

December 30 2021: Setup Rust env, Near-cli and testnet account

December 31 2021: Research on Rust and the near-sdk

January 01 2022: Having some fun :)

January 03 2022: Implementing the Smart Contract + unit tests and playing with the calls

January 07 2022: Fixing a unit test and introducing upgradability

January 08 2022: Making the contract upgradable using the different methods described on the doc (subaccounts, migration method and enums, this last one didn't go that well...)

## To sum up:

I really enjoied playing aroung with the near-sdk and doing my research on what are the best practices. Of course there is plenty of room for improvement but I am happy with the result so far. I have the feeling that after a review from you guys I can do some optimizations on the code.

Small concern from my side: the events.
I followed an example form the Near doc and by now, the result looks fine in the block explorer but I have the feeling that it can be done in a better way. I hope you can give me some detailed feedback regarding this.

## [Deprecated] Links to block explorer:

Contract/Account:
https://explorer.testnet.near.org/accounts/to-do-tasks.dexterdev8.testnet

## Reference links:

https://github.com/mikedotexe/rust-contract-upgrades

https://nomicon.io/ChainSpec/Upgradability.html

https://www.near-sdk.io/reducing-contract-size/examples

https://doc.rust-lang.org/nightly/core/option/enum.Option.html

https://docs.rs/near-sdk/latest/near_sdk/collections/struct.LookupMap.html

https://github.com/near/near-sdk-rs/tree/master/examples/status-message

https://www.near-sdk.io/contract-structure/collections#storage-space

https://www.near-sdk.io/contract-structure/near-bindgen

https://docs.near.org/docs/develop/contracts/rust/near-sdk-rs

https://docs.near.org/docs/tutorials/contracts/nfts/events
