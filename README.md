# Lottery Smart Contract on NEAR testnet in Rust

## Description

This contract implements simple lottery game with a 1 NEAR prize for the first person to guess the winning number. 

Contract in src/lib.rs provides methods to 
    1. Set a winning lottery number 
    2. Guess the winning lottery number

## Setup
Follow the guide provided by NEAR Org (https://www.near-sdk.io/zero-to-hero/basics/set-up-skeleton)

## Interact w Contract
### 1. Login to NEAR CLI 
* [Rust SDK Book](https://www.near-sdk.io/)
### 2. Create a subaccount
```
near create-account lottery.ryantan.testnet --masterAccount ryantan.testnet
```
*reminder: substitute the testnet accounts with your own

### 3. Compile smart contract
```
./build.sh
```
### 4. Deploy smart contract using batch actions
```
near deploy lottery.ryantan.testnet --wasmFile res/lottery.wasm \
  --initFunction 'new' \
  --initArgs '{"winner": "fe2592b42a727e977f055947385b709cc82b16b9a87f88c6abf3900d65d0cdc3"}'
```
*reminder: 'winner' is the sha256 hash of the number chosen. The value above is the sha256 hash of 4321

### 5. Guess the winning number

Wrong guess attempt:
```
near call lottery.ryantan.testnet guess_number '{"guess": "1234"}' --accountId ryantan.testnet
```
Correct guesss attempt:
```
near call lottery.ryantan.testnet guess_number '{"guess": "4321"}' --accountId ryantan.testnet
```
*reminder: substitute the testnet accounts with your own

**Get more info at:**
* [Rust SDK Book](https://www.near-sdk.io/)
* [Template](https://github.com/near-examples/rust-template)
