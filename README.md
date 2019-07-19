# kraken-price
A Rust cli tool to query Kraken's current btc-usd price, with doge added as a bonus. Other coins beware.

## Quick start
```bash
kraken-price btc
kraken-price doge
kraken-price --help
```

## Compile the binaries yourself
Kraken-price is written in Rust, so you'll need to grab a [Rust installation](https://www.rust-lang.org/) in order to compile it. Once you have Rust installed on your computer, you can easily use the tool by compiling the binaries yourself like so;
1. clone this repository locally;
2. enter the repo;
3. use `cargo build --release`;
4. move the compiled binary in your PATH (for MacOS and Linux, the default directory for those binaries would be `usr/local/bin`);
5. run using `kraken-price` in your command line, or rename the file your preferred command if you wish to use a different one.

```bash
git clone https://github.com/thunder-B/kraken-price
cd kraken-price
cargo build --release
cd ./target/release/bin/
cp ./kraken-price /usr/local/bin/    # MacOS and Linux

# test it out!
kraken-price dentacoin
```

## Download the binaries from crates.io
The binary crate is also released on [crates.io](https://crates.io/), which means you can install it directly from cargo (note that the binaries will then be built in release mode and put into your `~/.cargo/bin/` directory);
```bash
cargo install kraken-price

# test it out!
kraken-price dentacoin
```

## Download the binaries from Github
The zipped binaries for all 3 main operating systems are also downloadable from the [releases](https://github.com/thunder-B/kraken-price/releases) page of the [project's github repository](https://github.com/thunder-B/kraken-price).