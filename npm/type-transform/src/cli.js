#!/usr/bin/env node

import { exec } from 'node:child_process';
import { fileURLToPath } from 'node:url';

import { getExePath } from './get-exe-path.js';

async function main() {
  return new Promise(resolve => {

    let args = process.argv.slice(2);
    args = args
      .filter(val => val.toLowerCase().trim() !== '--yes')
      .filter(val => val.toLowerCase().trim() !== '--no')
      .filter(val => val.toLowerCase().trim() !== '--y');

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
