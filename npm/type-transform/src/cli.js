#!/usr/bin/env node

import { exec } from 'node:child_process';
import { pathToFileURL } from 'node:url';

import { getExePath } from './index.js';

async function main() {
  return new Promise(resolve => {

    let args = process.argv.slice(2);
    args = args
      .filter(val => val.toLowerCase().trim() !== '--yes')
      .filter(val => val.toLowerCase().trim() !== '--no')
      .filter(val => val.toLowerCase().trim() !== '--y');

    const exePath = getExePath();

    const cmd = `${pathToFileURL(exePath).href} ${args.join(' ')}`;

    exec(cmd, (err, stdout, stderr) => {
      if (err) {
        console.log(stderr);
      } else {
        console.log(stdout);
      }
      resolve();
    });
  });
}

main();
