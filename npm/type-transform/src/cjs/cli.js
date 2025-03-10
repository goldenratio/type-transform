#!/usr/bin/env node
"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const node_child_process_1 = require("node:child_process");
const node_url_1 = require("node:url");
const get_exe_path_js_1 = require("./get-exe-path.js");
async function main() {
    return new Promise(resolve => {
        let args = process.argv.slice(2);
        args = args
            .filter(val => val.toLowerCase().trim() !== '--yes')
            .filter(val => val.toLowerCase().trim() !== '--no')
            .filter(val => val.toLowerCase().trim() !== '--y');
        const exePath = (0, get_exe_path_js_1.getExePath)();
        const cmd = `${(0, node_url_1.fileURLToPath)(exePath)} ${args.join(' ')}`;
        (0, node_child_process_1.exec)(cmd, (err, stdout, stderr) => {
            if (err) {
                console.log(stderr);
            }
            else {
                console.log(stdout);
            }
            resolve();
        });
    });
}
main();
