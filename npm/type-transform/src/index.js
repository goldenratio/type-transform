import { exec } from 'node:child_process';
import { fileURLToPath } from 'node:url';
import { arch as getArch, platform as getPlatform } from 'node:os';

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
export function transform(srcFilePath, outFilePath, options = {}) {
  return new Promise((resolve) => {

    const exePath = getExePath();
    const args = [srcFilePath, `--out ${outFilePath}`];
    if (typeof options?.banner === 'string') {
      args.push(`--banner ${options.banner}`);
    }

    if (typeof options?.footer === 'string') {
      args.push(`--footer ${options.footer}`);
    }

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

/**
 * @throws
 * @return {string}
 */
export function getExePath() {
  const platform = getPlatform();
  const arch = getArch();

  let os = platform;
  let extension = '';

  if (platform === 'win32' || platform === 'cygwin') {
    os = 'windows';
    extension = '.exe';
  }

  const binPath = `@goldenratio/type-transform-${os}-${arch}/bin/type-transform${extension}`;

  try {
    return import.meta.resolve(binPath);
  } catch (err) {
    throw new Error(`Cannot find type transform binary! ${binPath}`, err);
  }
}
