# type-transform

> Convert TypeScript types to other language types

## Usage

```
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
