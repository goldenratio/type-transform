"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.transform = transform;
const node_child_process_1 = require("node:child_process");
const node_url_1 = require("node:url");
const get_exe_path_js_1 = require("./get-exe-path.js");
/**
 * Transform TypeScript types to Swift/Kotlin types
 *
 * @param {string} srcFilePath - The path to the source Typscript file to be transformed.
 * @param {string} outFilePath - The path where the transformed file should be saved.
 * @param {Object} [options={}] - Optional parameters for transformation.
 * @param {string} [options.banner] - An optional banner string to be added to the output.
 * @param {string} [options.footer] - An optional footer string to be added to the output.
 * @returns {Promise<{ success: boolean }>} - A promise that resolves with an object indicating success or failure.
 */
function transform(srcFilePath, outFilePath, options = {}) {
    return new Promise((resolve) => {
        const exePath = (0, get_exe_path_js_1.getExePath)();
        const args = [srcFilePath, `--out ${outFilePath}`];
        if (typeof options?.banner === 'string') {
            args.push(`--banner ${options.banner}`);
        }
        if (typeof options?.footer === 'string') {
            args.push(`--footer ${options.footer}`);
        }
        const cmd = `${(0, node_url_1.fileURLToPath)(exePath)} ${args.join(' ')}`;
        (0, node_child_process_1.exec)(cmd, (err) => {
            if (err) {
                resolve({ success: false });
            }
            else {
                resolve({ success: true });
            }
        });
    });
}
