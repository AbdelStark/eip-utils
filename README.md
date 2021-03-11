# EIP validator

[![license](https://img.shields.io/badge/license-Apache--2.0-blue)](https://github.com/abdelhamidbakhta/eip-utils)
[![ci status](https://github.com/abdelhamidbakhta/eip-utils/workflows/ci/badge.svg)](https://github.com/abdelhamidbakhta/eip-utils/actions)

A command line utility tool which provides [Ethereum Improvement
Proposals](https://eips.ethereum.org) features.

## Getting Started

### Install

```console
git clone https://github.com/abdelhamidbakhta/eip-utils.git
cargo install --path=eip-utils eip-utils
```

### Usage

```console
eip-utils --help
```

### Commands

#### EIP-1559

##### eip1559-compute-basefee

```
Compute base fee from parent block header

USAGE:
    eip-utils eip1559-compute-basefee --parent-base-fee <parent-base-fee> --parent-gas-used <parent-gas-used> --parent-target-gas-used <parent-target-gas-used>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --parent-base-fee <parent-base-fee>                  The value of the parent block base fee
        --parent-gas-used <parent-gas-used>                  The value of the parent block gas used
        --parent-target-gas-used <parent-target-gas-used>    The value of the parent block target gas used
```

```console
# Help
eip-utils help eip1559-compute-basefee
# Example 
eip-utils eip1559-compute-basefee --parent-base-fee=1000000000 --parent-gas-used=10000000 --parent-target-gas-used=5000000
# Output
1125000000
```