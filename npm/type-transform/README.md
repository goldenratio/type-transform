# @goldenratio/type-transform

>  Convert TypeScript types to Swift/Kotlin types

### Install

> npm i --save-dev @goldenratio/type-transform

https://www.npmjs.com/package/@goldenratio/type-transform

### Node API Usage

```js
import { transform } from '@goldenratio/type-transform';

const { success } = await transform('./ts-files/hello-world.ts', './gen/HelloWorld.swift');
console.log(success);
```

### NPX Usage

```sh
npx @goldenratio/type-transform@latest --yes ./ts-files/hello-world.ts --out ./gen/HelloWorld.swift
```

### Releasing NPM Package(s)

- Run `node ./build-dependencies.js` - This will build OS specific npm packages
- Run `./publish.sh <VERSION>` - This will publish those binaries and root package to NPM
