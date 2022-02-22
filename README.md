# Solana NFT

![sol](./sol.jpg)

## References

- [Getting Started with MetaPlex](https://docs.metaplex.com/token-metadata/getting-started)
- [File System Wallet](https://docs.solana.com/wallet-guide/file-system-wallet)
- [Install the Solana Tool Suite](https://docs.solana.com/cli/install-solana-cli-tools)

## Installation

#### Install the Solana release v1.9.8 on your machine:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

#### Confirm you have the desired version of solana installed:
```bash
solana --version
```
```bash
solana-cli 1.9.8 (src:4ebeb336; feat:2191737503)
```

## Concepts

- Wallet: TODO
- SPL Token: TODO
- Wallet: TODO

## Connecting a wallet

#### Use Solana's command-line tool solana-keygen to generate keypair files.
```bash
solana-keygen new --outfile key.json
```
```bash
Generating a new keypair
[...]
Wrote new keypair to key.json
=============================================================================
pubkey: GRbqKQ332wWMsFU43N3VSY9EhhPsNKZh3sszhXdsQSR3
=============================================================================
Save this seed phrase and your BIP39 passphrase to recover your new keypair:
mixture code leopard relax nice debris truth close differ hurry donor balcony
=============================================================================
```

#### Retrieve the private key from the json file.
```bash
cat key.json
```
```bash
[41,88,225,51,18,0,227,73,125,103,15,76,72,117,150,92,125,181,75,247,58,25,210,119,4,84,64,158,138,249,239,209,229,44,161,17,139,218,26,171,96,204,62,111,122,15,174,13,108,108,173,221,27,98,122,203,2,74,235,143,157,13,195,174]
```

#### Verify you hold the private key for a given address.
```bash
solana-keygen verify GRbqKQ332wWMsFU43N3VSY9EhhPsNKZh3sszhXdsQSR3 key.json
```
```bash
Verification for public key: GRbqKQ332wWMsFU43N3VSY9EhhPsNKZh3sszhXdsQSR3: Success
```

#### Set the local environment to authenticate its requests with the new wallet.
```bash
solana config set --keypair key.json 
```
```bash
Config File: /home/martinalejandrocastroalvarez/.config/solana/cli/config.yml
RPC URL: https://api.mainnet-beta.solana.com 
WebSocket URL: wss://api.mainnet-beta.solana.com/ (computed)
Keypair Path: key.json 
Commitment: confirmed 
```

#### Add SOL to your wallet on devnet
```bash
solana airdrop 1 GRbqKQ332wWMsFU43N3VSY9EhhPsNKZh3sszhXdsQSR3 
```
```bash
Signature: 2RkG5e9TZozHQFVqGBaB5pwpSwo1CEiZYud4C5BwFSybN3S8gjQp41hUBqQ533fa7WcLukP5uHE9SnB9e86Ni8Nu
```

#### Open the wallet on the Solana explorer

- [mainet](https://explorer.solana.com/address/GRbqKQ332wWMsFU43N3VSY9EhhPsNKZh3sszhXdsQSR3)
- [devnet](https://explorer.solana.com/address/GRbqKQ332wWMsFU43N3VSY9EhhPsNKZh3sszhXdsQSR3?cluster=devnet)

#### Connect to devnet
```bash
solana config set --url devnet
```
```bash
Config File: /home/martinalejandrocastroalvarez/.config/solana/cli/config.yml
RPC URL: https://api.devnet.solana.com 
WebSocket URL: wss://api.devnet.solana.com/ (computed)
Keypair Path: key.json 
Commitment: confirmed 
```

## Creating a new token

#### Create a new token.
```bash
spl-token create-token --decimals 10
```
```bash
Creating token C9r7VaDTBrgm8vPFB7BCjEVhXD3kW5vRT2PEgezdyjNQ
Signature: 2pFYQVVH24LWeHRT3FVr2PzTSPTDXWY5gtobFw7LGSwAKcNMyVWnWFzrP8xPdh5xdn64M925sckdqRjZKdB9RBYK
```

#### Open the new token on the Solana explorer

- [mainet](https://explorer.solana.com/address/C9r7VaDTBrgm8vPFB7BCjEVhXD3kW5vRT2PEgezdyjNQ)
- [devnet](https://explorer.solana.com/address/C9r7VaDTBrgm8vPFB7BCjEVhXD3kW5vRT2PEgezdyjNQ?cluster=devnet)

#### Create an account on your wallet to hold the new token
```bash
spl-token create-account C9r7VaDTBrgm8vPFB7BCjEVhXD3kW5vRT2PEgezdyjNQ
```
```bash
Creating account 9u9qHHstJC1vMSKvwEhTZ6pk2afSQgRAQpysKsVoA6j1
Signature: MwaXx7Pf4Xvs5uiUQkn2vM71vxzZTNqWMUkQS2SK35X8RCe1mvvE75rBYx5w2AUVwrZLLJHqkYQVRHawtLspdSv
```

#### Mint token.
```bash
spl-token mint C9r7VaDTBrgm8vPFB7BCjEVhXD3kW5vRT2PEgezdyjNQ 300
```
```bash
Minting 300 tokens
  Token: C9r7VaDTBrgm8vPFB7BCjEVhXD3kW5vRT2PEgezdyjNQ
  Recipient: 9u9qHHstJC1vMSKvwEhTZ6pk2afSQgRAQpysKsVoA6j1
```

#### Disable minting to set the token supply
```bash
spl-token authorize C9r7VaDTBrgm8vPFB7BCjEVhXD3kW5vRT2PEgezdyjNQ mint --disable
```
```bash
Updating C9r7VaDTBrgm8vPFB7BCjEVhXD3kW5vRT2PEgezdyjNQ
  Current mint authority: GRbqKQ332wWMsFU43N3VSY9EhhPsNKZh3sszhXdsQSR3
  New mint authority: disabled
```

#### Check the token supply
```bash
spl-token supply C9r7VaDTBrgm8vPFB7BCjEVhXD3kW5vRT2PEgezdyjNQ
```
```bash
300
```

## Creating a new NFT 

#### Create the new NFT.
```bash
spl-token create-token --decimals 0
```
```bash
Creating token CNwdEBCLHJN5sUDwydDMjJdKmNA88KSi16N4pFuxbsNL
Signature: FhkHuzejrW6goicoBSz5FLjwddy9VAYqgcqeQiWmwd8X2YUMWFz1NzftFr7aAERZpc9EJzcQyLyQHx1JE2NtmYd
```

#### Create an account on your wallet to hold the NFT
```bash
spl-token create-account CNwdEBCLHJN5sUDwydDMjJdKmNA88KSi16N4pFuxbsNL
```
```bash
Creating account 5mVvtcbLEYyf1HJuC7twrUEffCqKGe63Cqi8wmf1UWs8
Signature: 5rAuxkdZL3AQ32xUgRVZ5Y2rQjV2xyBhG9tN9Tw9rSSouUBd4VqGsKKuv47VFceyqjsCgULzNjm5obAL3huqD5h3
```

#### Mint 10 editions.
```bash
spl-token mint CNwdEBCLHJN5sUDwydDMjJdKmNA88KSi16N4pFuxbsNL 10
```
```bash
Minting 10 tokens
  Token: CNwdEBCLHJN5sUDwydDMjJdKmNA88KSi16N4pFuxbsNL
  Recipient: 5mVvtcbLEYyf1HJuC7twrUEffCqKGe63Cqi8wmf1UWs8
```

#### Disable minting to set the token supply
```bash
spl-token authorize CNwdEBCLHJN5sUDwydDMjJdKmNA88KSi16N4pFuxbsNL mint --disable
```
```bash
Updating CNwdEBCLHJN5sUDwydDMjJdKmNA88KSi16N4pFuxbsNL
  Current mint authority: GRbqKQ332wWMsFU43N3VSY9EhhPsNKZh3sszhXdsQSR3
  New mint authority: disabled
```

#### Open the NFT on the Solana explorer

- [mainet](https://explorer.solana.com/address/CNwdEBCLHJN5sUDwydDMjJdKmNA88KSi16N4pFuxbsNL)
- [devnet](https://explorer.solana.com/address/CNwdEBCLHJN5sUDwydDMjJdKmNA88KSi16N4pFuxbsNL?cluster=devnet)






```bash
git clone https://github.com/metaplex-foundation/metaplex.git
cd metaplex
cd js && yarn install && yarn bootstrap
cd packages/cli
```

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```






```bash
git clone https://github.com/metaplex-foundation/metaplex.git
cd metaplex
cd js && yarn install && yarn bootstrap
cd packages/cli
```

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

