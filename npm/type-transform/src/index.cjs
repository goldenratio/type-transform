const { exec } = require('node:child_process');
const { pathToFileURL } = require('node:url');
const { arch: getArch, platform: getPlatform } = require('node:os');

/**
 * Transform TypeScript types to Swift/Kotlin types
 *
 * @param {string} srcFilePath - The path to the source TypeScript file to be transformed.
 * @param {string} outFilePath - The path where the transformed file should be saved.
 * @param {Object} [options={}] - Optional parameters for transformation.
 * @param {string} [options.banner] - An optional banner string to be added to the output.
 * @param {string} [options.footer] - An optional footer string to be added to the output.
 * @returns {Promise<{ success: boolean }>} - A promise that resolves with an object indicating success or failure.
 */
function transform(srcFilePath, outFilePath, options = {}) {
  return new Promise((resolve) => {
    const exePath = getExePath();
    const args = [srcFilePath, `--out ${outFilePath}`];

    if (typeof options?.banner === 'string') {
      args.push(`--banner ${options.banner}`);
    }

    if (typeof options?.footer === 'string') {
      args.push(`--footer ${options.footer}`);
    }

    const cmd = `${pathToFileURL(exePath).href} ${args.join(' ')}`;
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
function getExePath() {
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
    return require.resolve(binPath);
  } catch (err) {
    throw new Error(`Cannot find type transform binary! ${binPath}`, { cause: err });
  }
}

module.exports = { transform, getExePath };
