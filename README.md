# type-transform

> Convert TypeScript types to Swift/Kotlin types

### Usage

```sh
type-transform <SRC> --out <OUT>
```

```
Arguments:
  <SRC>  Input file name

Options:
  -o, --out <OUT>        The output file. Target language is inferred from file extension
  -b, --banner <BANNER>  A banner to be added to the generated file, this can be a package path for "kotlin", a custom auto code generated message or a comment block such as a license for the code
  -f, --footer <FOOTER>  A footer to be added to the generated file, this can be something like a comment block for a license or just a fun easter egg
  -h, --help             Print help
  -V, --version          Print version

```

Example:
```sh
type-transform hello-world.ts --out hello-world.swift
```

### Supported Target Languages

- Swift (.swift)
- Kotlin (.kt)


### Build

```sh
cargo build --release
```

### Install

#### Pre-built Binaries
Checkout releases for binaries,
https://github.com/goldenratio/type-transform/releases
