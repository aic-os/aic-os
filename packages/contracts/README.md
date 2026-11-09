# AIC Contracts

Smart contracts for AIC-OS.

## Requirements
- Foundry 1.9.0+ (orge --version)

## Setup
Dependencies are not committed -- install after cloning:
    cd packages/contracts
    forge install foundry-rs/forge-std

## Build & Test
    forge build
    forge test

## Layout
- src/    contracts
- test/   Foundry tests
- script/ deploy scripts
