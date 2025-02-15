# type-transform

> Convert TypeScript types to other language types

## Usage

```sh
type-transform <SRC> --out <OUT>
```

```
Arguments:
  <SRC>  Input file name

Options:
  -o, --out <OUT>  The output file. Target language is inferred from file extension
  -h, --help       Print help
  -V, --version    Print version

```

Example:
```sh
type-transform hello-world.ts --out hello-world.swift
```

### Supported Target Languages

- Swift
- Kotlin


## Build

```sh
cargo build --release
```

## Install

### Pre-built Binaries
Checkout releases for binaries,
https://github.com/goldenratio/type-transform/releases
