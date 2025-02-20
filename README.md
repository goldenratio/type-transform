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
      --out <OUT>        The output file. Target language is inferred from file extension
      --banner <BANNER>  A banner to be added to the generated file, this can be a package path for "kotlin", a custom auto code generated message or a comment block such as a license for the code
      --footer <FOOTER>  A footer to be added to the generated file, this can be something like a comment block for a license or just a fun easter egg
  -h, --help             Print help
  -V, --version          Print version
```

Example:
```sh
type-transform hello-world.ts --out hello-world.swift
```

Example Usage of a Banner:

```sh
#!/usr/bin/env bash

set -e

DATE=$(date +%Y-%m-%dT%H:%M:%S%z)


type-transform ts-files/hello-world.ts --out gen/hello-world.kt --banner "// Hello World\n// This code was auto generated at ${DATE} \npackage com.github.goldenratio\n"

```
### Supported Target Languages

- Swift (.swift)
- Kotlin (.kt)


### Build

```sh
cargo build --release
```

## Install

### Pre-built Binaries
Checkout releases for binaries,
https://github.com/goldenratio/type-transform/releases

### NPM
https://www.npmjs.com/package/@goldenratio/type-transform


## Releasing Type-Transform

```
cargo release <VERSION LEVEL> --execute --no-publish
```

Where `<VERSION LEVEL>` is one of `major`, `minor`, or `patch`

Next you need to manually make the release in github from the tag. This will kick off the build process
to build all the releases assets and store them on the release in github. 
