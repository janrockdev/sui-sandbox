# SUI Sandpit

## Introduction

The repo contains a few examples of SUI applications written using the **Rust SDK**. The examples come with `README.md` files explaining step-by-step key SUI and blockchain contexts, references to more documentation, as well as instructions for configuring the environments and running them.

## Examples

- [01](/src/01_transaction/): How to send SUI coin from one wallet to another.

## Installation

You will need a working Rust installation with Cargo.
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Local SUI in Docker/WSL
```
apt update
apt upgrade -y
apt-get install -y curl git-all cmake gcc libssl-dev pkg-config libclang-dev libpq-dev build-essential wget net-tools

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
(echo; echo 'eval "$(/home/linuxbrew/.linuxbrew/bin/brew shellenv)"') >> /root/.bashrc
eval "$(/home/linuxbrew/.linuxbrew/bin/brew shellenv)"
brew install gcc

brew install sui
sui start
```