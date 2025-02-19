import { arch as getArch, platform as getPlatform } from 'os';

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
