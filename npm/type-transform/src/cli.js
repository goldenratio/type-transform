#!/usr/bin/env node

import { exec } from 'node:child_process';
import { fileURLToPath } from 'node:url';

import { getExePath } from './get-exe-path.js';

async function main() {
  return new Promise(resolve => {

    const args = process.argv.slice(2);
    console.log('cli args: ', args);

    const exePath = getExePath();

    const cmd = `${fileURLToPath(exePath)} ${args.join(' ')}`;
    exec(cmd, (err) => {
      if (err) {
        resolve({ success: false });
      } else {
        resolve({ success: true });
      }
    });
  });

}

main();
