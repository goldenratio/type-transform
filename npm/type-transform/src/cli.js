#!/usr/bin/env node

import { exec } from 'node:child_process';
import { fileURLToPath } from 'node:url';

import { getExePath } from './get-exe-path.js';

async function main() {
  return new Promise(resolve => {

    let args = process.argv.slice(2);
    console.log('cli args: ', args);

    const firstArg = args[0].toLowerCase().trim();
    if (firstArg === '-y' || firstArg === '--yes' || firstArg === '--no') {
      args.shift();
    }
    console.log('cli args: ', args);

    const exePath = getExePath();

    const cmd = `${fileURLToPath(exePath)} ${args.join(' ')}`;
    exec(cmd, (err, _, stderr) => {
      if (err) {
        throw new Error(stderr);
      } else {
        // success
      }
      resolve();
    });
  });
}

main();
