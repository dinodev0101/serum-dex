# serum-dex

[![Build Status](https://travis-ci.com/project-serum/serum-dex.svg?branch=master)](https://travis-ci.com/project-serum/serum-dex)
[![Discord Chat](https://img.shields.io/discord/739225212658122886?color=blueviolet)](https://discord.com/channels/739225212658122886)
[![License](https://img.shields.io/github/license/project-serum/serum-dex?color=blue)](https://opensource.org/licenses/Apache-2.0)

## Deploying the DEX

### Run unit tests

```
./do.sh test dex
```

### Compile the dex binary

```
./do.sh build dex
```

### Deploy the dex to the configured solana cluster

```
DEX_PROGRAM_ID="$(solana deploy dex/target/bpfel-unknown-unknown/release/serum_dex.so --use-deprecated-loader | jq .programId -r)"
```

## Run the fuzz tests

```
cd dex
cargo install cargo-fuzz
cargo fuzz run multiple_orders
```

## Using the client utility
```
cd crank

# read the autogenerated help text
cargo run -- help

# supported options are localnet, mainnet, testnet, devnet
CLUSTER=localnet

# verify that you have SOL balances for gas
KEYPAIR=~/.config/solana/id.json
solana balance -k $KEYPAIR

# run the demo script (this is mostly a smoke test)
cargo run -- $CLUSTER whole-shebang $KEYPAIR $DEX_PROGRAM_ID

# list a market with the default tick size and minimum quantity.
# if both assets have 6 decimals, this will be a quantity of 1 and a tick size of 0.01
COIN_MINT="..."
PRICE_CURRENCY_MINT="..."
cargo run -- $CLUSTER list-market $KEYPAIR $DEX_PROGRAM_ID --coin-mint $COIN_MINT --pc-mint $PRICE_CURRENCY_MINT
```

## First-time setup
```
# Building the dex
sudo apt-get install -y pkg-config build-essential python3-pip jq
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
curl -sSf https://raw.githubusercontent.com/solana-labs/solana/v1.3.9/install/solana-install-init.sh | sh -s - v1.3.9
export PATH="/home/ubuntu/.local/share/solana/install/active_release/bin:$PATH"

git clone https://github.com/project-serum/serum-dex
cd serum-dex
./do.sh update
./do.sh build dex

# run a solana cluster. in a new shell:
git clone https://github.com/solana-labs/solana --branch v1.3.10
cd solana
sudo apt-get install -y libssl-dev libudev-dev zlib1g-dev llvm clang
cargo build --release
export RUST_LOG=solana_runtime::system_instruction_processor=trace,solana_runtime::message_processor=info,solana_bpf_loader=debug,solana_rbpf=debug
NDEBUG=1 ./run.sh

# Deploy the dex to our cluster (in the old shell)
solana config set -u http://127.0.0.1:8899
solana-keygen new
solana airdrop 100
DEX_PROGRAM_ID="$(solana deploy dex/target/bpfel-unknown-unknown/release/serum_dex.so --use-deprecated-loader | jq .programId -r)"
CLUSTER=localnet
KEYPAIR=~/.config/solana/id.json

# run the demo script (this is mostly a smoke test)
cargo run -- $CLUSTER whole-shebang $KEYPAIR $DEX_PROGRAM_ID
```
