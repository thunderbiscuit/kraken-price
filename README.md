# kraken-price
A Rust cli tool to query Kraken's current btc-usd price, with doge added as a bonus. Other coins beware.

## Quick start
```bash
kraken-price btc
kraken-price doge
kraken-price --help
```

## Compile the binaries yourself
If you have _Rust_ installed on your computer, you can easily use the tool by compiling the binaries yourself;
1. clone this repository locally
2. enter the repo
3. use `cargo run`
4. move the compiled binary to your `usr/local/bin` directory (MacOS and Linux)
5. run using `kraken-price` in your command line, or rename the file your preferred command if you wish to use a different one.

```bash
git clone https://github.com/thunder-B/kraken-price
cd kraken-price
cargo build --release
cd ./target/release/bin/
cp ./kraken-price /usr/local/bin/

# test it out!
kraken-price dentacoin
```

## Download the binaries from crates.io
The binary crate is also released on [crates.io](https://crates.io/), which means you can install it directly from cargo (note that the binaries will then be built in release mode and put into your `~/.cargo/bin/` directory);
```bash
cargo install kraken-price
```

## Download the binaries from Github
The zipped binaries for all 3 main operating systems are also downloadable from the [releases](https://github.com/thunder-B/kraken-price/releases) page of the [project's github repository](https://github.com/thunder-B/kraken-price).