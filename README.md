# Gazer

The Gazer is a small cli-tool for fetching, storing and exporting value-transfers (ERC20 and Eth) from the Ethereum blockchain. 
It is written in Rust and uses the [Otterscan](https://github.com/wmitsuda/otterscan) RPC extensions to [Erigon](https://github.com/ledgerwatch/erigon) to fetch its data. 

## Features
- Collect value transfers for a given address (incoming and outgoing calls as per `ots_search_transactions_after`)
  - Fetches eth transfers and erc20 transfers for the transactions fetched
- Store the value transfers in a local database (microKV) for later use
- Collect account details, such as nonce, code, storage, etc. Useful for figuring out if wallet (EOA or contract wallet) or a contract
- Export the collected data to a CSV file

## Requirements
Requires a running [Erigon](https://github.com/ledgerwatch/erigon) node with the [Otterscan](https://github.com/wmitsuda/otterscan) (ots) turned on.
```bash
ERIGON_FLAGS="--http.api=eth,erigon,engine,web3,net,debug,trace,txpool,ots"
```

## Usage

Currently, the tool uses `./data` as the data directory. This will be configurable in the future.

```bash
stats           
get-tx
get-account
export
  accounts
  txs
collect
  user
  tx
  populate-txs
  populate-accounts
  update
remove
  user
  all-txs
```

### Examples
```bash
# Collect value transfers for a given address (or list of addresses)
gazer collect --rpc-url http://localhost:8545 user <ADDRESS>

# Collect value transfer for a specific transaction
gazer collect --rpc-url http://localhost:8545 tx <TX_HASH>

# Populate database with missing content for transactions where we have "met" the hash
gazer collect --rpc-url http://localhost:8545 populate-txs

# Populate database with missing account details
gazer collect --rpc-url http://localhost:8545 populate-accounts

# Export collected accounts to a CSV file
gazer export accounts --path accounts.csv

# Export collected value transfers to a CSV file
gazer export txs --path transfers.csv
```