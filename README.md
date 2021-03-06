<img src="docs/logo.svg" width="200px" align="right" />

## NEAR Protocol - scalable and usable blockchain

![Build status](https://img.shields.io/gitlab/pipeline/nearprotocol/nearcore.svg)
<a href="https://discord.gg/gBtUFKR">![Discord](https://img.shields.io/discord/490367152054992913.svg)</a>
[![dependency status](https://deps.rs/repo/github/nearprotocol/nearcore/status.svg)](https://deps.rs/repo/github/nearprotocol/nearcore)

NEAR Protocol is a new smart-contract platform that delivers scalability and usability.

Through sharding it aims to linearly scale with number of validation nodes on the network.

Leveraging WebAssembly, TypeScript, more sane contract management, ephemeral accounts and many other advancement, NEAR
makes using blockchain protocol for developers and consumers way easier compared to competitors.

## Quick start

[Check out quick start documentation](https://docs.nearprotocol.com/#/quick_start), specifically:
  - [Running DevNet](https://docs.nearprotocol.com/#/quick_start#run--interact-with-devnet)
  - [Running TestNet](https://docs.nearprotocol.com/#/quick_start#running-testnet-locally)
  - [Build an ERC-20 contract](https://docs.nearprotocol.com/#/tutorials/erc20)
  
Develop and deploy contracts without any setup required using [NEARStudio](https://studio.nearprotocol.com):

[![NEAR Studio Demo](https://github.com/nearprotocol/NEARStudio/blob/master/demos/guest_book.gif)](https://studio.nearprotocol.com)



## Status

Project is currently under heavy development. Please see Issues and Milestones to checkout the current progress and work items.

High level milestones:

 - [x] DevNet: a tool with fully working State Transition + WebAssembly.
 - [ ] AlphaNet: Multi-node smart-contract platform.
 - [ ] BetaNet: Added economics and enchanced security.
 - [ ] MVB: Added goverannce module, ready for launching as Minimum Viable Blockchain.
 - [ ] Shard chains: Support for scalable sharded blockchain.

## Development

### Setup rust

```bash
$ curl https://sh.rustup.rs -sSf | sh
$ rustup component add clippy-preview
```

You may need to activate the environment via `. ~/.cargo/env` to use `cargo`.


### Install dependencies

Mac OS:
```bash
brew install protobuf
```

Ubuntu:
```bash
apt-get install protobuf-compiler
```

### Build & Run from source code

```bash
# Download NEAR Core code.
git clone https://github.com/nearprotocol/nearcore
cd nearcore
```

It will build the first time and then run:

```bash
cargo run
```

or

```bash
cargo run --package=devnet
```

 ### Testing

In order to run tests currently, you must setup `pynear`:

```bash
cd pynear
# sudo may be required if you are not testing with a python virtual environment
python setup.py develop
```

### Logging

For runnable apps (devnet, nearcore, etc.), you can use
the `--log-level` option to configure the log level across all internal crates.
You can also use the `RUST_LOG` environment variable, with `env_logger`
[semantics](https://docs.rs/env_logger/0.6.0/env_logger/#enabling-logging)
to override the log level for specific targets. `RUST_LOG` can also be used in
integration tests which spawn runnable apps.

Example:
```bash
$ RUST_LOG=runtime=debug cargo run -- --log-level warn
```

To add new target (e.g. `info!(target: "my target", "hello")`), 
add the desired target to the list in `node/cli/src/service.rs` in `configure_logging` function.

### Contributions

If you are planning to contribute, there are few more things to setup

#### Setup git hooks

```bash
./scripts/setup_hooks.sh
```

#### Setup rustfmt for your editor (optional)
Installation instructions [here](https://github.com/rust-lang-nursery/rustfmt#running-rustfmt-from-your-editor)

#### Lints
We currently use [clippy](https://github.com/rust-lang-nursery/rust-clippy) to enforce certain standards.
This check is run automatically during CI builds, and in a `pre-commit`
hook. You can run do a clippy check with `./scripts/run_clippy.sh`.

