#!/usr/bin/env node

import { spawn } from 'node:child_process';

import { getExePath } from './get-exe-path.js';

async function main() {
  const args = process.argv.slice(2);
  console.log('cli args: ', args);
  const exePath = getExePath();

  const child = spawn(exePath, args, { stdio: 'inherit' });
  child.on('close', (code) => {
    if (code !== 0) {
      process.exit(1);
    }
  });
}

main();
