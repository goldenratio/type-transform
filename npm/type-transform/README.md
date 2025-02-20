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

- Run `node ./build-bin.js` from parent folder - This will assemble OS specific binaries
- Run `./publish-bin.sh` - This will publish those binaries to npm
- Bump versions in `package.json` (current folder) and perform `npm i`
- Git commit those changes
- Run `npm publish`
