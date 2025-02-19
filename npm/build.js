#!/usr/bin/node

import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { exec } from 'node:child_process';
import { pipeline } from 'stream/promises';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const DOWNLOAD_DIR = path.join(__dirname, 'downloads');

const GITHUB_API = 'https://api.github.com/repos/goldenratio/type-transform/releases/latest';
const interestedAssets = [
  "x86_64-unknown-linux-musl.tar.gz",
  // "aarch64-apple-darwin.tar.gz",
  "x86_64-apple-darwin.zip"
];

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
      const validAsset = interestedAssets.some(v => asset.name.indexOf(v) >= 0 && asset.name.indexOf('sha256') === -1);
      if (validAsset) {
        console.log('Downloading: ', asset.name);
        const destArchiveFilePath = path.join(DOWNLOAD_DIR, asset.name);
        await downloadFile(asset.browser_download_url, destArchiveFilePath);
        const destFolderPath = destArchiveFilePath.replace('.tar.gz', '')
          .replace('.zip', '')
          .replace(`type-transform_${release.tag_name}_`, '')
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
  console.log('releaseVersion: ', releaseVersion);
}

main();
