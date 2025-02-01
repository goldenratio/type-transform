# type-transform

> Convert TypeScript types to other language types

## Usage

> type-transform --out <OUT> <SRC>

```
Arguments:
  <SRC>  Input file name

Options:
  -o, --out <OUT>  The output file. Determines --lang and --top-level
  -h, --help       Print help
  -V, --version    Print version

```

Example:
```sh
type-transform -o hello-world.swift hello-world.ts
```

### Supported Languages

- Swift
- Kotlin
