"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.getExePath = getExePath;
const os_1 = require("os");
/**
 * @throws
 * @return {string}
 */
function getExePath() {
    const platform = (0, os_1.platform)();
    const arch = (0, os_1.arch)();
    let os = platform;
    let extension = '';
    if (platform === 'win32' || platform === 'cygwin') {
        os = 'windows';
        extension = '.exe';
    }
    const binPath = `@goldenratio/type-transform-${os}-${arch}/bin/type-transform${extension}`;
    try {
        return import.meta.resolve(binPath);
    }
    catch (err) {
        throw new Error(`Cannot find type transform binary! ${binPath}`, err);
    }
}
