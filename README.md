# CosmWasm Plus

**CosmWasm Plus** is a fork of the [CosmWasm/cw-plus](https://github.com/nezha90/cw-plus), created to offer optimized smart contract solutions for specific application scenarios. Our core work focuses on extending and customizing the original features to meet more complex business needs.

An important contract we have developed is `cw1-deposit`. It fully leverages the performance and features of the Titan chain and uses its test token, tnt4, as the basis for a complete staking and rewards system. This contract provides users with a secure and transparent way to stake their tokens and earn rewards according to predefined rules.

## Prerequisites
Before starting, make sure you have rustup along with a recent rustc and cargo version installed. Currently, we are testing on 1.58.1+.

And you need to have the wasm32-unknown-unknown target installed as well.

You can check that via:
```shell
rustc --version
cargo --version
rustup target list --installed
# if wasm32 is not listed above, run this
rustup target add wasm32-unknown-unknown
```


## Compiling
Now that you created your custom contract, make sure you can compile and run it before making any changes. Go into the repository and do:
```shell
# this will produce a wasm build in ./target/wasm32-unknown-unknown/release/YOUR_NAME_HERE.wasm
cargo wasm

# this runs unit tests with helpful backtraces
RUST_BACKTRACE=1 cargo unit-test


# auto-generate json schema
cargo schema
```

## Generating JSON Schema
While the Wasm calls (`instantiate`, `execute`, `query`) accept JSON, this is not enough information to use it. We need to expose the schema for the expected messages to the clients. You can generate this schema by calling `cargo schema`, which will output 4 files in `./schema`, corresponding to the 3 message types the contract accepts, as well as the internal `State`.

These files are in standard json-schema format, which should be usable by various client side tools, either to auto-generate codecs, or just to validate incoming json wrt. the defined schema.



## Preparing the Wasm bytecode for production
Before we upload it to a chain, we need to ensure the smallest output size possible, as this will be included in the body of a transaction. We also want to have a reproducible build process, so third parties can verify that the uploaded Wasm code did indeed come from the claimed rust code.

To solve both these issues, we have produced `rust-optimizer`, a docker image to produce an extremely small build output in a consistent manner. The suggest way to run it is this:


```shell
docker run --rm -v "$(pwd)":/code \
--mount type=volume,source="$(basename "$(pwd)")_cache",target=/target \
--mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
cosmwasm/optimizer:0.16.0
```
Or, If you're on an arm64 machine, you should use a docker image built with arm64.

```shell
docker run --rm -v "$(pwd)":/code \
--mount type=volume,source="$(basename "$(pwd)")_cache",target=/target \
--mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
cosmwasm/optimizer-arm64:0.16.0
```
We must mount the contract code to `/code`. You can use an absolute path instead of `$(pwd)` if you don't want to `cd` to the directory first. The other two volumes are nice for speedup. Note the `/target` cache is unique for each contract being compiled to limit interference, while the registry cache is global.

This is rather slow compared to local compilations, especially the first compile of a given contract. The use of the two volume caches is very useful to speed up following compiles of the same contract.

This produces an `artifacts` directory with a `PROJECT_NAME.wasm`, as well as `checksums.txt`, containing the Sha256 hash of the wasm file. The wasm file is compiled deterministically (anyone else running the same docker on the same git commit should get the identical file with the same Sha256 hash). It is also stripped and minimized for upload to a blockchain (we will also gzip it in the uploading process to make it even smaller).

