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

![wallet.png](./wallet.png)

#### Connect to the devnet cluster
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

## Creating a token

#### Create a new token.
```bash
spl-token create-token --decimals 10
```
```bash
Creating token AqoJM91CTkXXhyx8qi5HJZGPaozRHc33zSyXz1EnTnWn 
Signature: 44CPkEej8Mz2okM41673XGWDmARpykn7gfFEzySSmj51vhYivmwkJycCVV1Xdno3MzexQQF5u8XK1e5fkbRhEmnd
```

#### Open the new token on the Solana explorer

- [mainet](https://explorer.solana.com/address/AqoJM91CTkXXhyx8qi5HJZGPaozRHc33zSyXz1EnTnWn)
- [devnet](https://explorer.solana.com/address/AqoJM91CTkXXhyx8qi5HJZGPaozRHc33zSyXz1EnTnWn?cluster=devnet)

![token.png](./token.png)

#### Create an account on your wallet to hold the NFT
```bash
spl-token create-account AqoJM91CTkXXhyx8qi5HJZGPaozRHc33zSyXz1EnTnWn
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

#### Mint token.
```bash
spl-token mint AqoJM91CTkXXhyx8qi5HJZGPaozRHc33zSyXz1EnTnWn 1000000
```
```bash
Minting 1000000 tokens
  Token: AqoJM91CTkXXhyx8qi5HJZGPaozRHc33zSyXz1EnTnWn
  Recipient: 6WbLiFB765NT3RXBUNxzvFkitPmiBuyDTLpiuP6G2Hh8
```

#### Disable minting to set the token supply
```bash
spl-token authorize AqoJM91CTkXXhyx8qi5HJZGPaozRHc33zSyXz1EnTnWn mint --disable
```
```bash
Updating AqoJM91CTkXXhyx8qi5HJZGPaozRHc33zSyXz1EnTnWn
  Current mint authority: GRbqKQ332wWMsFU43N3VSY9EhhPsNKZh3sszhXdsQSR3
  New mint authority: disabled
```

#### Check the token supply
```bash
spl-token supply AqoJM91CTkXXhyx8qi5HJZGPaozRHc33zSyXz1EnTnWn
```
```bash
300
```

#### Check your balance
```bash
spl-token balance AqoJM91CTkXXhyx8qi5HJZGPaozRHc33zSyXz1EnTnWn
```
```bash
1000000
```

#### Open the account on the Solana explorer to check your balance

- [mainet](https://explorer.solana.com/address/6WbLiFB765NT3RXBUNxzvFkitPmiBuyDTLpiuP6G2Hh8)
- [devnet](https://explorer.solana.com/address/6WbLiFB765NT3RXBUNxzvFkitPmiBuyDTLpiuP6G2Hh8?cluster=devnet)

![account.png](./account.png)

## Creating an NFT 

#### Create the new NFT.
```bash
spl-token create-token --decimals 0
```
```bash
Creating token 8kjeYR9e5R8D1DjZjsRtVktCK7xkbGytqxgwXn842dyB
Signature: 45bawtJRWxLRcPRqnqe2mxsmXosnnR3wfXi2kFntppYZzTe8naphHH5yLLiWz7mx528NkqUwUiEafkmF1uTwXJdj
```

#### Create an account on your wallet to hold the NFT
```bash
spl-token create-account 8kjeYR9e5R8D1DjZjsRtVktCK7xkbGytqxgwXn842dyB
```
```bash
Creating account y1tHVi5L4yxBXYts3pDXr66biCcbaTeqtTuiKdEfLUd
Signature: 4LgztnskmdowCN7tnKsXkfP3r39dyqfnS7RmByXFbFHNwyFrPHx6kKKefR5YHcRFrWb7H6GpiBZFFwqJioZqXWrh
```

#### Mint 10 editions.
```bash
spl-token mint 8kjeYR9e5R8D1DjZjsRtVktCK7xkbGytqxgwXn842dyB 10
```
```bash
Minting 10 tokens
  Token: 8kjeYR9e5R8D1DjZjsRtVktCK7xkbGytqxgwXn842dyB
  Recipient: y1tHVi5L4yxBXYts3pDXr66biCcbaTeqtTuiKdEfLUd
```

#### Disable minting to set the token supply
```bash
spl-token authorize 8kjeYR9e5R8D1DjZjsRtVktCK7xkbGytqxgwXn842dyB mint --disable
```
```bash
Updating 8kjeYR9e5R8D1DjZjsRtVktCK7xkbGytqxgwXn842dyB
  Current mint authority: GRbqKQ332wWMsFU43N3VSY9EhhPsNKZh3sszhXdsQSR3
  New mint authority: disabled
```

#### Open the NFT on the Solana explorer

- [mainet](https://explorer.solana.com/address/8kjeYR9e5R8D1DjZjsRtVktCK7xkbGytqxgwXn842dyB)
- [devnet](https://explorer.solana.com/address/8kjeYR9e5R8D1DjZjsRtVktCK7xkbGytqxgwXn842dyB?cluster=devnet)

![nft.png](./nft.png)

#### Check your balance
```bash
spl-token balance AqoJM91CTkXXhyx8qi5HJZGPaozRHc33zSyXz1EnTnWn
```
```bash
1000000
```

#### Open the account on the Solana explorer to check your balance

- [mainet](https://explorer.solana.com/address/y1tHVi5L4yxBXYts3pDXr66biCcbaTeqtTuiKdEfLUd)
- [devnet](https://explorer.solana.com/address/y1tHVi5L4yxBXYts3pDXr66biCcbaTeqtTuiKdEfLUd?cluster=devnet)

![account2.png](./account2.png)









```bash
git clone https://github.com/metaplex-foundation/metaplex.git
cd metaplex
cd js && yarn install && yarn bootstrap
cd packages/cli
```

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```


