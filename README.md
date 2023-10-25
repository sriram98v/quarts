## Binary
CLI tool to generate quartets from newick files

## Installation
```shell
cargo install --git=https://github.com/sriram98v/quarts
```

## Usage
Write all quartets of file to .quart file

Usage: quarts --num <NUM_T> <SRC_FILE>

Arguments:
  <SRC_FILE>  Source file with input tree (Will automatically use last tree of file is more than one is present)

Options:
  -n, --num <NUM_T>  Number of threads
  -h, --help         Print help
  -V, --version      Print version
