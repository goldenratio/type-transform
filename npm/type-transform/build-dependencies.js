#!/usr/bin/node

import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { exec } from 'node:child_process';
import { pipeline } from 'node:stream/promises';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const DOWNLOAD_DIR = path.join(__dirname, '../', 'downloads');
const PACKAGE_NAME = 'type-transform';
const GITHUB_API = `https://api.github.com/repos/goldenratio/${PACKAGE_NAME}/releases/latest`;

async function downloadLatestRelease() {
  let releaseVersion = undefined;
  try {
    await ensureDirectory(DOWNLOAD_DIR);
    const response = await fetch(GITHUB_API);
    if (!response.ok) {
      throw new Error(`GitHub API error: ${response.statusText}`);
    }
    const release = await response.json();
    console.log(`Latest Release: ${release.name} (${release.tag_name})`);
    releaseVersion = release.tag_name.replace('v', '');

    if (!release.assets.length) {
      console.log('No assets found in the latest release.');
      return;
    }

    for (const asset of release.assets) {
      const validAsset = asset.name.indexOf('sha256sum') === -1;
      if (validAsset) {
        console.log('Downloading: ', asset.name);
        const destArchiveFilePath = path.join(DOWNLOAD_DIR, asset.name);
        await downloadFile(asset.browser_download_url, destArchiveFilePath);
        const destFolderPath = destArchiveFilePath.replace('.tar.gz', '')
          .replace('.zip', '')
          .replace(`${PACKAGE_NAME}_${release.tag_name}_`, '')
          .replace('x86_64', 'x64');

        await extractArchive(destArchiveFilePath, destFolderPath);
        fs.unlinkSync(destArchiveFilePath);
      }
    }

    console.log('Download complete.');
  } catch (err) {
    console.error(err);
  }
  return { releaseVersion };
}

async function downloadFile(url, destFilPath) {
  const assetResponse = await fetch(url);
  if (!assetResponse.ok) {
    console.error(`Failed to download ${url}`);
    return;
  }

  const fileStream = fs.createWriteStream(destFilPath);
  await pipeline(assetResponse.body, fileStream);
}

async function ensureDirectory(dir) {
  if (fs.existsSync(dir)) {
    fs.rmSync(dir, { recursive: true, force: true });
  }
  fs.mkdirSync(dir, { recursive: true });
}

async function extractArchive(src, destFolder) {
  await ensureDirectory(destFolder);
  return new Promise((resolve, reject) => {
    let cmd = '';
    if (src.includes('.tar')) {
      cmd = `tar -xzf ${src} -C ${destFolder}`;
    } else {
      cmd = `unzip -o ${src} -d ${destFolder}`;
    }

    exec(cmd, (err, stdout, stderr) => {
      if (err) {
        reject(`Error: ${stderr}`);
        return;
      }
      resolve(`Extraction complete: ${stdout}`);
    });

  });
}

async function main() {
  const { releaseVersion } = await downloadLatestRelease();
  // console.log('releaseVersion: ', releaseVersion);
  // const releaseVersion = '0.0.13';
  const nodePackages = [
    { pkgSuffix: 'linux-x64', os: 'linux', cpu: 'x64', binDir: path.resolve(DOWNLOAD_DIR, 'x64-unknown-linux-musl') },
    { pkgSuffix: 'darwin-arm64', os: 'darwin', cpu: 'arm64', binDir: path.resolve(DOWNLOAD_DIR, 'aarch64-apple-darwin') },
    { pkgSuffix: 'darwin-x64', os: 'darwin', cpu: 'x64', binDir: path.resolve(DOWNLOAD_DIR, 'x64-apple-darwin') },
    { pkgSuffix: 'windows-x64', os: 'win32', cpu: 'x64', binDir: path.resolve(DOWNLOAD_DIR, 'x64-pc-windows-gnu') },
  ];

  for (const pkg of nodePackages) {
    const packageName = `${PACKAGE_NAME}-${pkg.pkgSuffix}`;
    console.log(`Generating node package: ${packageName}`);
    const dirPath = path.resolve(__dirname, '../', packageName);
    await ensureDirectory(dirPath);

    const packageJSONTemplateFile = path.resolve('../package.json.tmpl');
    let data = fs.readFileSync(packageJSONTemplateFile, { encoding: 'utf8' });
    data = data
      .replace('${node_pkg}', packageName)
      .replace('${version}', releaseVersion)
      .replace('${node_os}', pkg.os)
      .replace('${node_cpu}', pkg.cpu);

    const destPackageJSONPath = path.resolve(dirPath, 'package.json');
    fs.writeFileSync(destPackageJSONPath, data, { encoding: 'utf8' });

    const binDir = path.resolve(dirPath, 'bin');
    await ensureDirectory(binDir);

    fs.cpSync(pkg.binDir, binDir, { recursive: true });
  }

  console.log(`Finished geenrating dependencies for version: ${releaseVersion}`);
}

main();
