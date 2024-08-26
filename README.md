## Passphrase Generator

```
A simple passphrase generator

Usage: passphrase_generator [OPTIONS]

Options:
  -w, --words <WORDS>            Number of words in the generated passphrase [default: 5]
  -c, --capitalize               Whether to capitalize the first letter in each word
  -n, --numbers                  Whether to append a number (0-9) to each word
  -s, --separator <SEPARATOR>    What string to use between words [default: -]
  -m, --max-length <MAX_LENGTH>  Maximum length of the generated passphrase
  -h, --help                     Print help
  -V, --version                  Print version
```

## Build Instructions

```
$ cargo build --release
```

## Run Instructions

```
$ ./target/release/passphrase_generator -h
```
