# Lottery Smart Contract on NEAR testnet in Rust

## Description

This contract implements simple lottery game. 

Contract in src/lib.rs provides methods to 
    1. Set a winning lottery number 
    2. Guess the winning lottery number

## Setup
Follow the guide provided by NEAR Org (https://www.near-sdk.io/zero-to-hero/basics/set-up-skeleton)

## Interact
1. Login to NEAR CLI 
2. Create a subaccount
```
near create-account lottery.<yourAcc>.testnet --masterAccount <yourAcc>.testnet
```
3. Compile smart contract
```
./build.sh
```
4. Deploy smart contract using batch actions
```
near deploy lottery.<yourAcc>.testnet --wasmFile res/lottery.wasm \
  --initFunction 'new' \
  --initArgs '{"winner": <winningNum>}'
```
5. Guess the winning number
```
near call lottery.ryantan.testnet guess_number '{"guess": <guessNum>}' --accountId <yourAcc>.testnet
```

**Get more info at:**
* [Rust SDK Book](https://www.near-sdk.io/)
* [Template](https://github.com/near-examples/rust-template)
