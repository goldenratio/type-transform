# @goldenratio/type-transform

>  Convert TypeScript types to Swift and Kotlin types

### Install

> npm i --save-dev @goldenratio/type-transform

https://www.npmjs.com/package/@goldenratio/type-transform

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
type-transform hello-world.ts --out HelloWorld.swift
```

Example Usage of a Banner:

```sh
#!/usr/bin/env bash

set -e

DATE=$(date +%Y-%m-%dT%H:%M:%S%z)


type-transform ts-files/hello-world.ts --out gen/HelloWorld.kt --banner "// Hello World\n// This code was auto generated at ${DATE} \npackage com.github.goldenratio\n"

```
### Supported Target Languages

- Swift (.swift)
- Kotlin (.kt)

### Example

```ts
// hello-world.ts

export interface Contract {
  readonly currency: Promise<Currency>;
  getFoo(): Currency;
}

enum Currency {
  USD = 10.2,
  EUR = 42.5
}
```

will be converted to,

```swift
// HelloWorld.swift

public protocol Contract {
  var currency: Currency { get async throws }
  func getFoo() -> Currency
}

enum Currency: Double, CaseIterable { 
  case USD = 10.2
  case EUR = 42.5
}
```

```kotlin
// HelloWorld.kt

package com.github.goldenratio

import kotlinx.coroutines.Deferred

interface Contract {
    val currency: Deferred<Currency>
    fun getFoo(): Currency
}

enum class Currency(val value: Double) { 
    USD(10.2)
    EUR(42.5)
}
```

### Node API Usage

```js
import { transform } from '@goldenratio/type-transform';

const { success } = await transform('./ts-files/hello-world.ts', './gen/HelloWorld.swift');
console.log(success);
```

### NPX Usage

```sh
npx @goldenratio/type-transform@latest ./ts-files/hello-world.ts --out ./gen/HelloWorld.swift
```

### Releasing NPM Package(s)

- Run `./build-dependencies.js` - This will build OS specific npm packages

- Run `./publish.sh` - This will publish all OS specific binaries and main package to NPM.
