# HashedBatchNFT

A CW721-compatible smart contract based on [cw721-base](https://github.com/CosmWasm/cw-nfts/tree/main/contracts/cw721-base).

---

## Key Features

*  NFTs contain `batch_date` and a list of `hashes`
*  Each hash must be unique across all NFTs
*  Only the contract owner can mint or burn tokens
*  Hashes are saved after a successful mint and removed after a burn

---

## Usage

### Build

```sh
cargo build
```

### Run tests

```sh
cargo test
```

---

## Example Mint Message

```json
{
  "mint": {
    "token_id": "batch-001",
    "owner": "your-address",
    "token_uri": null,
    "extension": {
      "batch_date": "2025-06-26",
      "hashes": [
        "hash1",
        "hash2"
      ]
    }
  }
}
```

---

## Example Burn Message

```json
{
  "burn": {
    "token_id": "batch-001"
  }
}
```

---

## Deployment

```sh
wasmd tx wasm instantiate <CODE_ID> '{"name":"HashedBatchNFT","symbol":"HASH","minter":"<your-address>"}' \
  --label "hashed-nft" --from <wallet> --amount 2stake
```

---


