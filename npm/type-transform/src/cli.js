#!/usr/bin/env node

import { exec } from 'node:child_process';
import { fileURLToPath } from 'node:url';

import { getExePath } from './get-exe-path.js';

async function main() {
  let args = process.argv.slice(2);
  console.log('cli args: ', args);

  if (args[0].toLowerCase() === '-y') {
    args.shift();
  }

  const exePath = getExePath();

  const cmd = `${fileURLToPath(exePath)} ${args.join(' ')}`;
  exec(cmd, (err, _, stderr) => {
    if (err) {
      throw new Error(stderr);
    } else {
      // success
    }
  });

}

main();
