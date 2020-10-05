# Welcome: Tari Fuzz Testing
This crate contains Tari fuzz tests and a cli app to run them.

## The crate helps with the following:
- Corpus generation with [proptest](https://github.com/altsysrq/proptest) 
- Fuzzing with different engines [Libfuzzer](http://llvm.org/docs/LibFuzzer.html), [AFL.RS](https://github.com/rust-fuzz/afl.rs), [Honggfuzz-rs](https://github.com/rust-fuzz/honggfuzz-rs)


## Prerequisites:
You need [Libfuzzer](http://llvm.org/docs/LibFuzzer.html) to get going (the fuzzer uses it by default). 

## Targets
This fuzzer is used to fuzz the tari core,  utilities and crypto crates. 


## Installing Engines
Libfuzzer
```sh
cargo install cargo-fuzz
```

AFL
```sh
cargo install afl
```

Hongfuzz
```sh
cargo install honggfuzz
```

## Setup
To get started, use the following commands:
```sh
mkdir fuzztari
cd fuzztari
git clone https://github.com/tari-project/tari
git clone https://github.com/tari-project/tari_utilities
git clone https://github.com/tari-project/tari-crypto
git clone https://github.com/the-mog/tari-fuzzer

cd tari-fuzzer
```

## Engines Directories
Fuzz engines are installed at the following directories:

Libfuzzer
```sh
./fuzzer-libfuzzer
```

AFL
```sh
./fuzzer-afl
```

Hongfuzz
```sh
./fuzzer-hongfuzz
```

## Corpus
The fuzzer generates corpus by default, you can set how many you want (default: 25).
To fuzz using your own corpus, create a directory under your fuzz engine using this naming convention: `corpus-{target-name}` then set the seeds count to 0 using the -n flag:
`cargo run fuzz -t <target> -n 0`

## Artifacts/Crashes
Artifacts are saved under each fuzzer's directory..

## Usage

List Available Engines
```sh
cargo run list-engines
```

List Available Targets
```sh
cargo run list-targets
```
Generate Corpus for Target
```sh
cargo run gen-corpus -t <target to use>
```

Fuzz a Specific Target using Default Settings
```sh
cargo run fuzz -t <target>
```
## Adding Fuzz Targets
To add a target, you need to edit two files:  
`fuzz_targets/mod.rs` and `src/corpus.rs`


## TODO
Add more fuzz tests.  
Code Coverage.  
Structure-aware fuzzing.

