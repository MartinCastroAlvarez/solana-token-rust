# Solana NFT

![sol](./sol.jpg)

## 1 Table of Contents

- [1 Table of Contents](#1-table-of-contents)
- [2 References](#2-references)
- [3 Fractals Marketplace](#3-fractals-marketplace)
- [4 Installation](#4-installation)
- [5 Running a Validator](#5-running-a-validator)
- [6 Connecting a Wallet](#6-connecting-a-wallet)
- [7 Creating a Token](#7-creating-a-token)
- [8 Creating an NFT](#8-creating-an-nft)
- [9 Listing the Token](#9-listing-the-token)
- [10 Transferring Tokens](#10-transferring-tokens)
- [11 Rust Development](#11-rust-development)
- [12 Deploying a Program](#12-deploying-a-program)
- [13 Approvals](#13-approvals)

## 2 References

- [Solana CLI Usage](https://docs.solana.com/cli/usage)
- [Getting Started with MetaPlex](https://docs.metaplex.com/token-metadata/getting-started)
- [File System Wallet](https://docs.solana.com/wallet-guide/file-system-wallet)
- [Install the Solana Tool Suite](https://docs.solana.com/cli/install-solana-cli-tools)
- [How To Transfer Eth Assets to Solana Using Wormhole ](https://blog.mercurial.finance/how-to-transfer-eth-assets-to-solana-using-wormhole-v2-514e728dc4fc)
- [Neon](https://docs.neon-labs.org/docs/intro)
- [Creating an NFT on Solana with Metaplex Candy Machine](https://dev.to/devindxdeveloper/creating-an-nft-on-solana-with-metaplex-candy-machine-4agp)
- [fractals.is](https://www.fractal.is/)
- [Solana Discord Channel](https://discord.com/invite/solana)
- [Token Metadata](https://docs.metaplex.com/token-metadata/getting-started)
- [Cargo Installation](https://doc.rust-lang.org/cargo/getting-started/installation.html)
- [Deploy a Program](https://docs.solana.com/cli/deploy-a-program)

## 3 Fractals Marketplace

#### 3.1 Visit [https://www.fractal.is/](https://www.fractal.is/)

![fractals1.png](./fractals1.png)

#### 3.2 Look at the latest NFTs

![fractals2.png](./fractals2.png)

#### 3.3 Visit [NEKO #591](https://www.fractal.is/4809210122469376/5074427943321600/AU89dkTtXt8GBaTt6e4LE35rQeztWaSS6CJ3Ny1XFn8)

![fractals3.png](./fractals3.png)

#### 3.4 Look at the transactions associated with the token

![fractals4.png](./fractals4.png)

#### 3.5 Visit the [Solana Explorer](https://explorer.solana.com/address/AU89dkTtXt8GBaTt6e4LE35rQeztWaSS6CJ3Ny1XFn8)

![fractals5.png](./fractals5.png)

#### 3.6 Look at the [Metadata](https://explorer.solana.com/address/AU89dkTtXt8GBaTt6e4LE35rQeztWaSS6CJ3Ny1XFn8/metadata)

![fractals6.png](./fractals6.png)

#### 3.7 Look at the [Instructions](https://explorer.solana.com/address/AU89dkTtXt8GBaTt6e4LE35rQeztWaSS6CJ3Ny1XFn8/instructions)

![fractals7.png](./fractals7.png)

## 4 Installation

### 4.1 [Python SDK](https://michaelhly.github.io/solana-py/)

Installing the SDK is not recommended. It is just a wrapper for the OS `solana-cli` which captures the output on STDOUT and transforms it into Python code. However, if the `solana-cli` version is incompatible, it does not work properly.

### 4.2 [moralis.io](https://moralis.io/moralis-releases-solana-api/)

It can can be used to interact with the Blockchain. For example:
```bash
curl -X GET "https://solana-gateway.moralis.io/account/devnet/GRbqKQ332wWMsFU43N3VSY9EhhPsNKZh3sszhXdsQSR3/balance?cluster=devnet" -H "X-Api-Key: sBpG7zBJbKgI7DXHUWnqYxxgWveqpoQZ0mbCqti0SnfV7GQxyoKneiIGyd9lMx4q"
```
```bash
{
    "lamports":"24982265600",
    "solana":"24.9822656"
}
```

![moralis.png](./moralis.png)

### 4.3 [Neon](https://neon-labs.org/)

This tool provides an abstraction layer which provides an Ethereum-like interface to the Solana network.

![neon.png](./neon.png)

### 4.4 Local Development

#### 4.4.1 Install the Solana release v1.9.8 on your machine
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
sh -c "$(curl -sSfL https://release.solana.com/v1.9.8/install)"
source $HOME/.cargo/env
```

#### 4.4.2 Confirm that Rust was installed
```bash
rustc --version
```
```bash
rustc 1.59.0 (9d1b2106e 2022-02-23)
```

#### 4.4.3 Confirm that Cargo was intsalled
```bash
cargo -V
```
```bash
cargo 1.59.0 (49d8809dc 2022-02-10)
```

#### 4.4.4 Confirm you have the desired version of solana installed
```bash
solana --version
```
```bash
solana-cli 1.9.8 (src:4ebeb336; feat:2191737503)
```

#### 4.4.5 Metaplex
```bash
git clone https://github.com/metaplex-foundation/metaplex.git
cd metaplex
git checkout v1.1.1
cd js && yarn install && yarn bootstrap
npm install -g typescript
```

## 5 Running a Validator

There is no need to run our own validator.

#### 5.1 Synchronous Commands

Most commands have this flag to wait for confirmation before returning.

```bash
spl-token transfer --help
```
```bash
[...]
    --no-wait   Return signature immediately after submitting the transaction,
                instead of waiting for confirmations.
[...]
```

#### 5.2 Watch for logs in real time. It generates ~10.000 lines in 5 seconds.
```bash
solana logs | tee test.log
```
```bash
Transaction executed in slot 116878215:
  Signature: 53gEBYVA453FhxButWmfZb8Nkj8c19eUKkqE9FMZZ1kHPHF5gXkmzZ5wPVFxVXu2qAzKDQKsgxfU5apzTpnufj1G
  Status: Ok
  Log Messages:
    Program gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s invoke [1]
    Program gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s consumed 2506 of 200000 compute units
    Program gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s success
    Program gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s invoke [1]
    Program gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s consumed 2496 of 200000 compute units
    Program gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s success
    Program gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s invoke [1]
    Program gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s consumed 2511 of 200000 compute units
    Program gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s success
    Program gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s invoke [1]
    Program gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s consumed 2494 of 200000 compute units
    Program gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s success
    Program gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s invoke [1]
    Program gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s consumed 2555 of 200000 compute units
    Program gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s success
    Program gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s invoke [1]
    Program gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s consumed 2535 of 200000 compute units
    Program gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s success
    Program gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s invoke [1]
    Program gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s consumed 2557 of 200000 compute units
    Program gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s success
    Program gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s invoke [1]
    Program gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s consumed 2537 of 200000 compute units
    Program gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s success
Transaction executed in slot 116878215:
  Signature: 3LvGbTSNeb6jZ5hhW7GgagXGzMg5DtrRuaRVeRqWaDQEJ9VzYjo9psrDX7yDEqYWw6pmETJVMGMKKhNp8Y68JKEP
  Status: Error processing Instruction 0: invalid program argument
  Log Messages:
    Program gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s invoke [1]
    Program gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s consumed 594 of 200000 compute units
    Program gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s failed: invalid program argument
```

## 6 Connecting a Wallet

#### 6.1 Use Solana's command-line tool solana-keygen to generate keypair files
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

#### 6.2 Retrieve the private key from the json file
```bash
cat key.json
```
```bash
[41,88,225,51,18,0,227,73,125,103,15,76,72,117,150,92,125,181,75,247,58,25,210,119,4,84,64,158,138,249,239,209,229,44,161,17,139,218,26,171,96,204,62,111,122,15,174,13,108,108,173,221,27,98,122,203,2,74,235,143,157,13,195,174]
```

#### 6.3 Verify you hold the private key for a given address
```bash
solana-keygen verify GRbqKQ332wWMsFU43N3VSY9EhhPsNKZh3sszhXdsQSR3 key.json
```
```bash
Verification for public key: GRbqKQ332wWMsFU43N3VSY9EhhPsNKZh3sszhXdsQSR3: Success
```

#### 6.4 Set the local environment to authenticate its requests with the new wallet
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

#### 6.5 Open the wallet on the Solana explorer

- [mainet](https://explorer.solana.com/address/GRbqKQ332wWMsFU43N3VSY9EhhPsNKZh3sszhXdsQSR3)
- [devnet](https://explorer.solana.com/address/GRbqKQ332wWMsFU43N3VSY9EhhPsNKZh3sszhXdsQSR3?cluster=devnet)

![wallet.png](./wallet.png)

#### 6.6 Connect to the devnet cluster
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

![clusters.png](./clusters.png)

#### 6.7 Add SOL to your wallet on devnet
```bash
solana airdrop 1 GRbqKQ332wWMsFU43N3VSY9EhhPsNKZh3sszhXdsQSR3
```
```bash
Signature: 2RkG5e9TZozHQFVqGBaB5pwpSwo1CEiZYud4C5BwFSybN3S8gjQp41hUBqQ533fa7WcLukP5uHE9SnB9e86Ni8Nu
```
More about [Signatures](https://coinyuppie.com/is-it-an-authorized-transfer-or-a-direct-steal-solana-authorized-phishing-event-analysis/):
```
Signature Diffusion Mechanism in Solana

In Solana, there is a signature diffusion mechanism. The user calls contract A, and the user in contract A has signed and approved it. When contract A calls contract B internally, the user’s signature will spread to contract B along with the cross-contract call. Therefore, in contract B, the user is also signed to approve. So there is a security risk here. When signing a malicious contract, the malicious contract obtains our signature, but it can do anything with our signature! ! ! ! ! ! !

In the above-mentioned theft incident, the user made a transaction with the malicious contract 3VtjHnDuDD1QreJiYNziDsdkeALMT6b2F9j3AXdL4q8v, which directly called the system contract to transfer the user’s SOL, because the signature spread to the system contract along with the call, so the system contract considered the transaction It is also approved and normal, so the user’s assets are transferred.
```

## 7 Creating a Token

#### 7.1 Create a new token
```bash
spl-token create-token --decimals 10 --owner ./key.json
```
```bash
Creating token AqoJM91CTkXXhyx8qi5HJZGPaozRHc33zSyXz1EnTnWn 
Signature: 44CPkEej8Mz2okM41673XGWDmARpykn7gfFEzySSmj51vhYivmwkJycCVV1Xdno3MzexQQF5u8XK1e5fkbRhEmnd
```

#### 7.1 Open the new token on the Solana explorer

- [mainet](https://explorer.solana.com/address/AqoJM91CTkXXhyx8qi5HJZGPaozRHc33zSyXz1EnTnWn)
- [devnet](https://explorer.solana.com/address/AqoJM91CTkXXhyx8qi5HJZGPaozRHc33zSyXz1EnTnWn?cluster=devnet)

![token.png](./token.png)

#### 7.2 Create an account on your wallet to hold the token
```bash
spl-token create-account AqoJM91CTkXXhyx8qi5HJZGPaozRHc33zSyXz1EnTnWn --owner ./key.json
```
```bash
Creating account 6WbLiFB765NT3RXBUNxzvFkitPmiBuyDTLpiuP6G2Hh8
Signature: 2JDWfThLe8dN1sCPvn8pUuwAgfH3ZLVnjW741pvvr3xiwSvWxtWoCGDxt3zzsCCNFMxhPyDrAY6Q5WoxwWw1dXUN
```
Trying to create multiple accounts for the same token generates the following error:
```bash
Creating account 6WbLiFB765NT3RXBUNxzvFkitPmiBuyDTLpiuP6G2Hh8
Error: Account already exists: 6WbLiFB765NT3RXBUNxzvFkitPmiBuyDTLpiuP6G2Hh8
```

#### 7.3 Mint token
```bash
spl-token mint AqoJM91CTkXXhyx8qi5HJZGPaozRHc33zSyXz1EnTnWn 1000000 --owner ./key.json
```
```bash
Minting 1000000 tokens
  Token: AqoJM91CTkXXhyx8qi5HJZGPaozRHc33zSyXz1EnTnWn
  Recipient: 6WbLiFB765NT3RXBUNxzvFkitPmiBuyDTLpiuP6G2Hh8
```

#### 7.4 Disable minting to set the token supply
```bash
spl-token authorize AqoJM91CTkXXhyx8qi5HJZGPaozRHc33zSyXz1EnTnWn mint --disable --owner ./key.json
```
```bash
Updating AqoJM91CTkXXhyx8qi5HJZGPaozRHc33zSyXz1EnTnWn
  Current mint authority: GRbqKQ332wWMsFU43N3VSY9EhhPsNKZh3sszhXdsQSR3
  New mint authority: disabled
```

#### 7.5 Check the token supply
```bash
spl-token supply AqoJM91CTkXXhyx8qi5HJZGPaozRHc33zSyXz1EnTnWn
```
```bash
300
```

#### 7.6 Check your balance
```bash
spl-token balance AqoJM91CTkXXhyx8qi5HJZGPaozRHc33zSyXz1EnTnWn --owner ./key.json
```
```bash
1000000
```

#### 7.7 Open the account on the Solana explorer to check your balance

- [mainet](https://explorer.solana.com/address/6WbLiFB765NT3RXBUNxzvFkitPmiBuyDTLpiuP6G2Hh8)
- [devnet](https://explorer.solana.com/address/6WbLiFB765NT3RXBUNxzvFkitPmiBuyDTLpiuP6G2Hh8?cluster=devnet)

![account.png](./account.png)

## 8 Creating an NFT 

#### 8.1 Create a [config file](./assets/0.json) according to the [Token Metadata Program](https://docs.metaplex.com/token-metadata/specification)
```javascript
{
  "name": "Nisman",
  "symbol": "",
  "description": "Lorem Ipsum Dolor",
  "image": "0.jpeg",
  "animation_image": "0.jpeg",
  "external_url": "0.jpeg",
  "attributes": [
    {
      "trait_type": "attack",
      "value": "4"
    },
    {
      "trait_type": "defense",
      "value": "3"
    },
    {
      "trait_type": "durability",
      "value": "47"
    },
    {
      "trait_type": "components",
      "value": "iron: 10; carbon: 1; wood: 2"
    }
  ]
}
```

#### 8.2 Create an [image file](./assets/0.jpeg)

![image](./assets/0.jpeg)

#### 8.3 Create a [config file](./candy.json) for the Candy Machine
```bash
{
    "price": 10.0,
    "number": null,
    "gatekeeper": null,
    "solTreasuryAccount": "GRbqKQ332wWMsFU43N3VSY9EhhPsNKZh3sszhXdsQSR3",
    "goLiveDate": "28 Feb 2022 00:00:00 GMT",
    "storage": "nft-storage",
    "awsS3Bucket": null,
    "noRetainAuthority": true,
    "noMutable": true,
    "splTokenAccount": null,
    "splToken": null,
    "endSettings": null,
    "whitelistMintSettings": null,
    "hiddenSettings": null,
    "ipfsInfuraProjectId": null,
    "ipfsInfuraSecret": null,
    "nftStorageKey": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJkaWQ6ZXRocjoweDA2MDI0RGM0QmY2MUJiZjVEQjU1RkRFNGM4QzFkMGU0OTE5ZTI0MjgiLCJpc3MiOiJuZnQtc3RvcmFnZSIsImlhdCI6MTY0NjA5MTAxNDU5NywibmFtZSI6ImVzdCJ9.d9Ftb5H2Wd-Gs-0Jzk1ehG0LtjQiu5G_DVlM8TU8sew"
}
```

#### 8.4 Use Metaplex to upload the config for the NFT(s)
```bash
npx ts-node ./metaplex/js/packages/cli/src/candy-machine-v2-cli.ts upload \
    --env devnet --keypair ./key.json --config-path ./candy.json ./assets/
eark``
```bash
wallet public key: GRbqKQ332wWMsFU43N3VSY9EhhPsNKZh3sszhXdsQSR3
Beginning the upload for 2 (img+json) pairs
started at: 1646001449493
initialized config for a candy machine with publickey: 8vjuNEaUwjJJTRcGg66yywYbBJUu3ipNRYvPRULwkxri
Uploading Size 0 { mediaExt: '.jpeg', index: '0' }
Uploading Size 1 { mediaExt: '.jpeg', index: '1' }
Done. Successful = true.
ended at: 2022-02-27T22:37:29.499Z. time taken: 00:00:00
```

#### 8.5 Verify the upload using Metaplex
```bash
npx ts-node ./metaplex/js/packages/cli/src/candy-machine-v2-cli.ts verify_upload \
    --env devnet --keypair ./key.json
```
```bash
wallet public key: GRbqKQ332wWMsFU43N3VSY9EhhPsNKZh3sszhXdsQSR3
Key size 10
Looking at key  0
Looking at key  1
Looking at key  2
Looking at key  3
Looking at key  4
Looking at key  5
Looking at key  6
Looking at key  7
Looking at key  8
Looking at key  9
uploaded (10) out of (10)
ready to deploy!
```

#### 8.6 Confirm the images have been uploaded to [nft.storage](https://nft.storage/files/)

![nft.storage](./storage.png)

#### 8.7 Mint one token using Metaplex
```bash
npx ts-node ./metaplex/js/packages/cli/src/candy-machine-v2-cli.ts mint_one_token \
    --env devnet --keypair ./key.json
```
```bash
mint_one_token finished 2oy2UeJ7ajuSTdDRCGGfewo2LfYtdNfm1Pyhm4WrTWyGY2TK8hrCYkTJ9eL3sbMd3FhQrvmVjXrRgunhCpYwiv6C
```

#### 8.9 Mint multiple tokens
```bash
npx ts-node ./metaplex/js/packages/cli/src/candy-machine-v2-cli.ts mint_one_token \
    --env devnet --keypair ./key.json
```
```bash
mint_one_token finished 2oy2UeJ7ajuSTdDRCGGfewo2LfYtdNfm1Pyhm4WrTWyGY2TK8hrCYkTJ9eL3sbMd3FhQrvmVjXrRgunhCpYwiv6C
```

#### 8.10 Checking your balance 
```bash
spl-token accounts --owner ./key.json
```
```bash
Token                                         Balance
---------------------------------------------------------------
57LrgeQk19vkfQcwt9i4kcazHEpdW8P7PVxwqXLfmJr3  1      
5cZqJmAhnuQQKBmvDousYTDzpAeM9KdPQTyVoJL4byEe  10     
8kjeYR9e5R8D1DjZjsRtVktCK7xkbGytqxgwXn842dyB  9      
AqoJM91CTkXXhyx8qi5HJZGPaozRHc33zSyXz1EnTnWn  1000000
C9r7VaDTBrgm8vPFB7BCjEVhXD3kW5vRT2PEgezdyjNQ  220300 
CNwdEBCLHJN5sUDwydDMjJdKmNA88KSi16N4pFuxbsNL  0      
DEpBz1FB1Jg4JiWU331vpu7kDuTnPiypvhGFWhz19yXB  1      
FoxTkih1Tb3emCVV65V7G3avbCk7UrKgpJzrGfgLTw8e  1      
g291tKH38BBYXQWeKMrkPCXu1LshymdETbN4u6BeSns   1      
```

#### 8.11 Open the Candy Machine output on the Solana explorer

- [mainet](https://explorer.solana.com/address/8vjuNEaUwjJJTRcGg66yywYbBJUu3ipNRYvPRULwkxri)
- [devnet](https://explorer.solana.com/address/8vjuNEaUwjJJTRcGg66yywYbBJUu3ipNRYvPRULwkxri?cluster=devnet)

![candy1.png](./candy1.png)
![candy2.png](./candy2.png)

#### 8.12 Check the NFT metadata
![Token8.png](./token8.png)
![Token metadata.png](./token_metadata.png)
```bash
{
  "key": 4,
  "updateAuthority": "GRbqKQ332wWMsFU43N3VSY9EhhPsNKZh3sszhXdsQSR3",
  "mint": "g291tKH38BBYXQWeKMrkPCXu1LshymdETbN4u6BeSns",
  "data": {
    "name": "Number #0008",
    "symbol": "NISMAN",
    "uri": "https://bafkreiamuicmrmmb5cti4sv5aftzo26qcrltezkbhaxrbytlww3tkr66t4.ipfs.dweb.link",
    "sellerFeeBasisPoints": 0,
    "creators": [
      {
        "address": "DyiNkgy7xRPsg5iM8zrZziCX2wmq3E4mhMfdAHJfNzSU",
        "verified": 1,
        "share": 0
      },
      {
        "address": "GRbqKQ332wWMsFU43N3VSY9EhhPsNKZh3sszhXdsQSR3",
        "verified": 0,
        "share": 100
      }
    ]
  },
  "primarySaleHappened": 1,
  "isMutable": 0,
  "editionNonce": 253
}
```

#### 8.13 Trying to create an account for the token fails because it is handled by Candy Machine
```bash
spl-token create-account g291tKH38BBYXQWeKMrkPCXu1LshymdETbN4u6BeSns --owner key.json
```
```bash
Creating account DYc1kfPiCp6qsVk7jbTrEuKcBmDYPFtYjsWaZqPbPxoz
Error: Account already exists: DYc1kfPiCp6qsVk7jbTrEuKcBmDYPFtYjsWaZqPbPxoz
```

#### 9 Listing the Token

#### 9.1 Clone the the [Token List](https://github.com/solana-labs/token-list) repository
```bash
git clone https://github.com/solana-labs/token-list
cd token-list/
```

#### 9.2 Add the media to the logos directory
```bash
mkdir assets/mainnet/AqoJM91CTkXXhyx8qi5HJZGPaozRHc33zSyXz1EnTnWn/
cp ~/Downloads/logo.png assets/mainnet/AqoJM91CTkXXhyx8qi5HJZGPaozRHc33zSyXz1EnTnWn/logo.png
git add assets/mainnet/AqoJM91CTkXXhyx8qi5HJZGPaozRHc33zSyXz1EnTnWn/logo.png
```

#### 9.3 Add your token to the token list
To add a new token, add another json block to the large tokens list in `src/tokens/solana.tokenlist.json` and submit a PR.
```bash
vim src/tokens/solana.tokenlist.json 
```
```bash
    {
      "chainId": 101,
      "address": "AqoJM91CTkXXhyx8qi5HJZGPaozRHc33zSyXz1EnTnWn",
      "symbol": "NISMAN",
      "name": "Nisman Coin",
      "decimals": 10,
      "logoURI": "https://github.com/MartinCastroAlvarez/ethereum-solidity-contract/raw/main/logo.png",      "tags": [
        "solana"
      ],
      "extensions": {
        "website": "https://github.com/MartinCastroAlvarez/ethereum-solidity-contract"
      }
    },
```

#### 9.4 Commit your changes and submit a pull request.
```bash
git commit -a -m 'Adding MP token'
git checkout -b MartinCastroAlvarezNisman
git push origin MartinCastroAlvarezNisman
```

#### 10 Transferring tokens

#### 10.1 Create another wallet 
```bash
solana-keygen new --outfile other.json
```
```bash
Generating a new keypair
For added security, enter a BIP39 passphrase
NOTE! This passphrase improves security of the recovery seed phrase NOT the
keypair file itself, which is stored as insecure plain text
BIP39 Passphrase (empty for none): 
Enter same passphrase again: 
Wrote new keypair to other.json
================================================================================
pubkey: AxHmduv298YFHUi6eTRhoXAWVdmEsRD4i44Ce9kD8ipv
================================================================================
Save this seed phrase and your BIP39 passphrase to recover your new keypair:
during wheat disorder pepper maple wise number together laptop poet narrow trend
================================================================================
```

#### 10.2 Retrieve the private key from the json file
```bash
cat other.json
```
```bash
[53,55,82,181,90,137,126,208,219,71,54,77,119,145,77,88,5,103,208,217,10,218,187,210,47,131,48,215,123,226,223,110,147,227,22,153,66,69,41,186,253,0,224,158,116,81,90,102,12,218,125,243,30,144,25,89,122,152,197,55,190,90,78,215]
```

#### 10.3 Add SOL to your wallet on devnet
```bash
solana airdrop 1 AxHmduv298YFHUi6eTRhoXAWVdmEsRD4i44Ce9kD8ipv
```
```bash
Signature: 32Gk4dp9YZ4jrx8rdvUUisyJWi9cNSppNW3c6EEmcizvryEQ69GjEBBE9u4nTXy1bjUBwgjBZpU7sZ9uQSADmt9C
```

#### 10.4 Fetching the wallet info
```bash
solana account AxHmduv298YFHUi6eTRhoXAWVdmEsRD4i44Ce9kD8ipv
```
```bash
Public Key: AxHmduv298YFHUi6eTRhoXAWVdmEsRD4i44Ce9kD8ipv
Balance: 1 SOL
Owner: 11111111111111111111111111111111
Executable: false
Rent Epoch: 270
```

#### 10.5 Create an account for the other account
```bash
spl-token create-account g291tKH38BBYXQWeKMrkPCXu1LshymdETbN4u6BeSns --owner other.json
```
```bash
Creating account H16QimPvJ5GBc7eXzp4S2RxvsWqEypfZRDodwDY1vmRR
Signature: 61tgcMu6pJHDubGUQ2NsVQVrf1AhnrLVCxiJ9DijjLiMHBdJoUi7jpyLZXDM1Rkmkv7cQpQJRXr3g4DsE4Ps6DLW
```

#### 10.6 Transfer a release into the new account
```bash
spl-token transfer g291tKH38BBYXQWeKMrkPCXu1LshymdETbN4u6BeSns 1 H16QimPvJ5GBc7eXzp4S2RxvsWqEypfZRDodwDY1vmRR --owner ./key.json
```
```bash
Transfer 1 tokens
  Sender: DYc1kfPiCp6qsVk7jbTrEuKcBmDYPFtYjsWaZqPbPxoz
  Recipient: H16QimPvJ5GBc7eXzp4S2RxvsWqEypfZRDodwDY1vmRR
Signature: 2BU6SgoxEdBKagiTtKRNDrQZ18Nf1UfJaJWVkCJFC8UzLmqzzXEgDCZuZgDfphZgrW7m3D4o2iz21cN3T2vGCrmd
```

#### 10.7 Check the new balance
```bash
spl-token balance g291tKH38BBYXQWeKMrkPCXu1LshymdETbN4u6BeSns --owner GRbqKQ332wWMsFU43N3VSY9EhhPsNKZh3sszhXdsQSR3
```
```bash
0
```
```bash
spl-token balance g291tKH38BBYXQWeKMrkPCXu1LshymdETbN4u6BeSns --owner AxHmduv298YFHUi6eTRhoXAWVdmEsRD4i44Ce9kD8ipv
```
```bash
1
```

#### 10.8 Allow another account to transfer the NFT on your behalf
```bash
spl-token approve H16QimPvJ5GBc7eXzp4S2RxvsWqEypfZRDodwDY1vmRR 1 GRbqKQ332wWMsFU43N3VSY9EhhPsNKZh3sszhXdsQSR3 --owner other.json
```
```bash
Approve 1 tokens
  Account: H16QimPvJ5GBc7eXzp4S2RxvsWqEypfZRDodwDY1vmRR
  Delegate: GRbqKQ332wWMsFU43N3VSY9EhhPsNKZh3sszhXdsQSR3
Signature: JF7N61Gu6DqAcan6vYhZN1ezCQRHkPRkyTjc2JutS72pSxM3ymuh4PNbf8P5q662hc7QPHrBvT8ZF6ksVsAvSG9
```

#### 10.9 Use your approval to transfer the NFT on behalf of the other account
```bash
spl-token transfer g291tKH38BBYXQWeKMrkPCXu1LshymdETbN4u6BeSns 1 GRbqKQ332wWMsFU43N3VSY9EhhPsNKZh3sszhXdsQSR3 --owner key.json --from H16QimPvJ5GBc7eXzp4S2RxvsWqEypfZRDodwDY1vmRR
```
```bash
Transfer 1 tokens
  Sender: H16QimPvJ5GBc7eXzp4S2RxvsWqEypfZRDodwDY1vmRR
  Recipient: GRbqKQ332wWMsFU43N3VSY9EhhPsNKZh3sszhXdsQSR3
  Recipient associated token account: DYc1kfPiCp6qsVk7jbTrEuKcBmDYPFtYjsWaZqPbPxoz
Signature: 4AHAyKDfqWp4gk9jJTKx8ctqJWxLjL87Z5bG1eNHzAbQVAWqbYNN3Cqytj6rJmPma4aJd7aR7EcmgdhaiKUA6g2M
```

#### 10.10 Check the balance of the sender
```bash
spl-token balance g291tKH38BBYXQWeKMrkPCXu1LshymdETbN4u6BeSns --owner GRbqKQ332wWMsFU43N3VSY9EhhPsNKZh3sszhXdsQSR3
```
```bash
0
```
```bash
spl-token balance g291tKH38BBYXQWeKMrkPCXu1LshymdETbN4u6BeSns --owner AxHmduv298YFHUi6eTRhoXAWVdmEsRD4i44Ce9kD8ipv
```
```bash
1
```

#### 10.11 Check the latest transactions associated with the token
```bash
solana transaction-history g291tKH38BBYXQWeKMrkPCXu1LshymdETbN4u6BeSns
```
```bash
2BU6SgoxEdBKagiTtKRNDrQZ18Nf1UfJaJWVkCJFC8UzLmqzzXEgDCZuZgDfphZgrW7m3D4o2iz21cN3T2vGCrmd
61tgcMu6pJHDubGUQ2NsVQVrf1AhnrLVCxiJ9DijjLiMHBdJoUi7jpyLZXDM1Rkmkv7cQpQJRXr3g4DsE4Ps6DLW
4qui7MS7TH4fqnEXn8DnhWohvnpoohoogyrMmWC8a5GtrdvSAN22Y3uGQtLEGVutPEUHLtH4oyzzbrAV8rr14Bfd
3 transactions found
```

#### 10.12 Check the latest transactions associated with the receiver
```bash
solana transaction-history AxHmduv298YFHUi6eTRhoXAWVdmEsRD4i44Ce9kD8ipv
```
```bash
61tgcMu6pJHDubGUQ2NsVQVrf1AhnrLVCxiJ9DijjLiMHBdJoUi7jpyLZXDM1Rkmkv7cQpQJRXr3g4DsE4Ps6DLW
gA14iUUjZvxMmnZEpab61Zixb1XUqghzFx2FAjn3BHkbbP4U8Y99oZsgp443cJAxn73PLwbR7xE1kNEuvmGGYRt
5eGabQattEckgeZsMdTJMyAJ51iZc853wu3x2VWyeH8j7Tycf51M1R6JH36qvWWPEjXJXuZqQ2YG842zVNjzrrVM
U2yxzLAx57NYA4fYNq7QY5GG2aV15qCWjNHUAh2cbM2DozjbHJzpFoxA6KBYr6XX3hRMpjj9L6n6pubi7hGEPgd
4HkrBdeYcQTA6ZEC14rkKyQYMUoF2UgBc5Foa3xP7Q8PZCWg7eVhUVZaA4cz8M1Y5ZGQX1VJSm2J5KTb522hR5sA
2Q2mKdjvMFfXuDuvwbwY4X7mZEJaUTKcwjXzAnarX2Q5PxbTYKmi2DsEdrFtYJZ2ARDvSrXLnWAC4hPg7o4Gcjnb
2Jwve2yM79EguUzoEg8TptomsgGaWLEJTy5uEbyB3q3cuADJZ2gYAFYSntZ8CE9TCDBGy9Ga151nKcRKFddFsmow
2csLCiSNxJEizfxsFEDVYwaHRB69jkxEk2ZjWHfnvDu8DppR3DJ3YZ1F6zNjbAGWWc7ER8oau4pxjzJUZxCqH4a2
6mZPD17kUyk1W5ZssNZ8bezPedznGKE5dfCPyeRgWbYm4ejQoNsjA1XUGotPFTaPNfnyNSCim3FsioMwYXLVCsY
5xjn4CAKryxJnA1dMZwonpQzvwsCegGj7hCt8iXbWB4ifuZ3hFKPd2j2GvcE1fVHPHbmJCaNhr3GJHxscYBYCTvq
UhR64tVJzEWPvAsVJJCnAKV7nSgc7ZMrPGyt28sYEKv26hGhjp8j7jK4wDAcif5BEKx5HvApSA6HdE4sqTGyfD3
11 transactions found
```

## 11 Rust Development

### 11.1 Hello World Example

#### 11.1.1 Create a new project
```bash
cargo new hello
cd hello
```
```bash
Created binary (application) `hello` package
```

#### 11.1.2 Edit the [Cargo.toml](./hello/Cargo.toml) manifest file
```bash
[package]
name = "hello"
version = "0.1.0"
edition = "2021"
[dependencies]
```

#### 11.1.3 Edit the [main.rs](./hello/src/main.rs) program file
```rust
fn main() {
    println!("Hello, world!");
}
```

#### 11.1.4 Compile and run the program
```bash
cargo run
```
```bash
Compiling hello v0.1.0 (/home/martinalejandrocastroalvarez/contract/hello)
Finished dev [unoptimized + debuginfo] target(s) in 1.50s
    Running `target/debug/hello
Hello, world!
```

### 11.2 Arguments Example

#### 11.2.1 Create a new project
```bash
cargo new args
cd args
```
```bash
Created binary (application) `args` package
```

#### 11.2.2 Edit the [Cargo.toml](./args/Cargo.toml) manifest file
```bash
[package]
name = "args"
version = "0.1.0"
edition = "2021"
[dependencies]
```

#### 11.2.3 Edit the [main.rs](./args/src/main.rs) program file
```rust
use std::env::{args, Args};
fn main() {
    let mut args: Args = args();
    let first: String = args.nth(1).unwrap();
    let second: String = args.nth(0).unwrap();
    let third: String = args.nth(0).unwrap();
    println!("Hello, {:?} {:?} {:?}!", first, second, third);
}
```

#### 11.2.4 Compile and run the program
```bash
cargo run -- lorem ipsum dolor
```
```bash
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/nisman lorem ipsum dolor`
Hello, "lorem" "ipsum" "dolor"!
```

### 11.3 Art Example

#### 11.3.1 Create a new project
```bash
cargo new art
cd art
```
```bash
Created binary (application) `art` package
```

#### 11.3.2 Edit the [Cargo.toml](./art/Cargo.toml) manifest file
```bash
[package]
name = "art"
version = "0.1.0"
edition = "2021"
[dependencies]
```

#### 11.3.3 Edit the [data.rs](./art/src/data.rs) program file which contains the Structs
```rust
#[derive(Debug, Clone)]
pub struct Person {
    pub name: String,
    pub phone: u32 
}

#[derive(Debug, Clone)]
pub enum Category {
    Image,
    Video
}

#[derive(Debug, Clone)]
pub struct Art {
    pub name: String,
    pub price: u64,
    pub owner: Person,
    pub creator: Person
}

impl Art {
    pub fn get_price(&self) -> u64 {
        self.price
    }
}
```

#### 11.3.4 Edit the [main.rs](./art/src/main.rs) program file
```rust
mod data;
use data::Art;
use data::Person;
use data::Category;

fn main() {
    let image: Art = Art {
        name: String::from("Dolor"),
        price: 128319823.try_into().unwrap(),
        creator: Person {
            name: String::from("Lorem"),
            phone: 123125.try_into().unwrap()
        },
        owner: Person {
            name: String::from("Ipsum"),
            phone: 58332394.try_into().unwrap()
        },
        category: Category::Image
    };
    let video: Art = Art {
        name: String::from("Dolor"),
        price: 128319823.try_into().unwrap(),
        creator: Person {
            name: String::from("Lorem"),
            phone: 123125.try_into().unwrap()
        },
        owner: Person {
            name: String::from("Ipsum"),
            phone: 58332394.try_into().unwrap()
        },
        category: Category::Video
    };
    println!("Image: {:?}", image);
    println!("Video: {:?}", video);
    println!("Image Price: {:?}", video.get_price());
    println!("Video Price: {:?}", image.get_price());
}
```

#### 11.4.5 Compile and run the program
```bash
cargo run -- lorem ipsum dolor
```
```bash
   Compiling art v0.1.0 (/home/martinalejandrocastroalvarez/contract/art)
    Finished dev [unoptimized + debuginfo] target(s) in 0.27s
     Running `target/debug/art`
Image: Art { name: "Dolor", price: 128319823, owner: Person { name: "Ipsum", phone: 58332394 }, creator: Person { name: "Lorem", phone: 123125 }, category: Image }
Video: Art { name: "Dolor", price: 128319823, owner: Person { name: "Ipsum", phone: 58332394 }, creator: Person { name: "Lorem", phone: 123125 }, category: Video }
Image Price: 128319823
Video Price: 128319823
```

## 12 Deploying a Program

#### 12.1 Create a new project
```bash
cargo new contract
cd contract
```
```bash
Created binary (application) `contract` package
```

#### 12.2 Edit the [Cargo.toml](./contract/Cargo.toml) manifest file

#### 12.3 Edit the [main.rs](./contract/src/main.rs) program file

#### 12.4 Creating a release
```bash
cargo build --release
```
```bash
Finished release [optimized] target(s) in 2m 40s
```

#### 12.5 After building the release, compile for Solana
```bash
rustc ./src/main.rs 
./main
```
```bash
TODO
```

## 13 Approvals

#### 13.1 Creating a token account
```
spl-token create-account AqoJM91CTkXXhyx8qi5HJZGPaozRHc33zSyXz1EnTnWn --owner ./other.json
```
```
Account: 9SiGoPXKt5K7gjFCMQa9w3YdbLDbPSme7XFAVywwHmeD
```

#### 13.2 Trasnferring some tokens
```
spl-token transfer AqoJM91CTkXXhyx8qi5HJZGPaozRHc33zSyXz1EnTnWn 100 9SiGoPXKt5K7gjFCMQa9w3YdbLDbPSme7XFAVywwHmeD --owner ./key.json
```
```
Signature: 59GeZ6yvnGJakjPeoJ3LeGQJKvZ3FsoD5QgSXAMdzvTEu8f21SbD9NKZ8vm6LPQn1Bzd7HqAcrNxnUArN8kyFR4q
```

#### 13.3 Approving 10 tokens
```
spl-token approve 9SiGoPXKt5K7gjFCMQa9w3YdbLDbPSme7XFAVywwHmeD 10 GRbqKQ332wWMsFU43N3VSY9EhhPsNKZh3sszhXdsQSR3 --owner other.json
```
```
Signature: 59GeZ6yvnGJakjPeoJ3LeGQJKvZ3FsoD5QgSXAMdzvTEu8f21SbD9NKZ8vm6LPQn1Bzd7HqAcrNxnUArN8kyFR4q
```

#### 13.4 Transferring those tokens
```
spl-token transfer AqoJM91CTkXXhyx8qi5HJZGPaozRHc33zSyXz1EnTnWn 9 GRbqKQ332wWMsFU43N3VSY9EhhPsNKZh3sszhXdsQSR3 --owner key.json --from 9SiGoPXKt5K7gjFCMQa9w3YdbLDbPSme7XFAVywwHmeD
```
```
Signature: 59GeZ6yvnGJakjPeoJ3LeGQJKvZ3FsoD5QgSXAMdzvTEu8f21SbD9NKZ8vm6LPQn1Bzd7HqAcrNxnUArN8kyFR4q
```

#### 13.5 Transferring more tokens than approved causes an error
```
spl-token transfer AqoJM91CTkXXhyx8qi5HJZGPaozRHc33zSyXz1EnTnWn 2 GRbqKQ332wWMsFU43N3VSY9EhhPsNKZh3sszhXdsQSR3 --owner key.json --from 9SiGoPXKt5K7gjFCMQa9w3YdbLDbPSme7XFAVywwHmeD
```
```
Signature: 59GeZ6yvnGJakjPeoJ3LeGQJKvZ3FsoD5QgSXAMdzvTEu8f21SbD9NKZ8vm6LPQn1Bzd7HqAcrNxnUArN8kyFR4q
```
