# Terp Event Attendance Hub

Tea-Hub is an NFT protocol that allows anyone to permissionlessly create digital TEA, as rewards for participants of events, or people who achieve certain accomplishments.

## Overview

### Contracts

The tea project consists of two smart contracts:

- `tea-hub` is where users create, edit, or mint new tea
- `tea-nft` is the non-fungible token that implements the `TEA-721` interface.

### Minting

Creation of new tea is permissionless. When creating a new tea, a fee is charged based on the amount of storage space it consumes. The fee rate, defined as uthiol per byte.

Each tea defines its own minting rule. There are three such rules to be chosen from:

- `by_minter` There is a designated minter, which can either be a human, a multisig, or another contract implementing custom minting logics. The minter can mint any amount of the tea to any user.
- `by_key` When creating the tea, the creator generates a private-public key pair, and provides the contract with the pubkey. The creator should then distribute the privkey off-chain. Any person who receives the privkey can mint an instance of the tea by submitting the signature of [a specified message](https://github.com/st4k3h0us3/tea/blob/363ab86d19c699202c7801f2d349af924c0cefb0/contracts/hub/src/helpers.rs#L16-L19) signed by the privkey. The privkey can be used many times, whereas each user can only mint once.
- `by_keys` Similar to the previous rule, but there are multiple privkeys, each can only be used once. Similarly, each user can only mint once.

Each tea can also optionally have a minting deadline and a max supply.

### Tokens

tea's are each identified by an integer number. The first tea ever to be created gets id #1, the second #2, and so on.

Each tea can be minted multiple times; the resulting **instances** of the tea are each identified by a **serial** number. For a given tea, the first instance to ever be minted gets serial #1, the second #2, and so on.

That is, each non-fungible token is identified by two numbers, the tea id and the serial number. The CW-721 `token_id` is defined by joining the two with a pipe character: `{id}|{serial}`. For example, the 420th instance of tea #69 has a `token_id` of `69|420`.

### Metadata

The metadata of tea are stored on-chain. However, the approach used by [`cw721-metadata-onchain`](https://github.com/CosmWasm/cw-nfts/tree/main/contracts/cw721-metadata-onchain) is not suitable for our use case. The said contract stores a separate copy of the metadata for each `token_id`. As instances of the same tea all have the same metadata, this is a huge waste of on-chain space.

Instead, only a single copy of the metadata is stored at the Hub contract. When a user queries the `nft_info` method on the NFT contract by providing a `token_id`, the NFT contract in turn queries the Hub contract for the metadata, and returns it to the user. In this way, we significantly reduce the contract's storage footprint.

### Purging

The Hub contract implements two methods, `purge_keys` and `purge_owners`, which allows anyone to delete certain contract data once they are no longer needed. This reduces the blockchain's state size and the burden for node operators.

## Deployment

### morocco-1

| Contract  | Address                                                                                                                                                                               |
| --------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Tea Hub | [`TBD`]() |
| Tea NFT | [`TBD`]() |

### 90u-2

| Contract  | Address                                                            |
| --------- | ------------------------------------------------------------------ |
| Tea Hub | `TBD` |
| Tea NFT | `TBD` |

## License

Contents of this repository are open source under [GNU General Public License v3](./LICENSE) or later.
