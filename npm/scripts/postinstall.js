const { execSync } = require("child_process");
const fs = require("fs");
const path = require("path");
const https = require("https");
const { createWriteStream } = require("fs");

const REPO = "agentmail-to/agentmail-cli";
const BINARY_NAME = "agentmail";

const PLATFORM_MAP = {
  darwin: "macos",
  linux: "linux",
  win32: "windows",
};

const ARCH_MAP = {
  arm64: "arm64",
  x64: "amd64",
  ia32: "386",
};

function getPlatformArch() {
  const platform = PLATFORM_MAP[process.platform];
  const arch = ARCH_MAP[process.arch];

  if (!platform || !arch) {
    console.error(
      `Unsupported platform: ${process.platform} ${process.arch}`
    );
    process.exit(1);
  }

  return { platform, arch };
}

function getAssetName(version, platform, arch) {
  const ext = platform === "linux" ? "tar.gz" : "zip";
  return `${BINARY_NAME}_${version}_${platform}_${arch}.${ext}`;
}

function fetch(url) {
  return new Promise((resolve, reject) => {
    https
      .get(url, { headers: { "User-Agent": "agentmail-cli-npm" } }, (res) => {
        if (res.statusCode >= 300 && res.statusCode < 400 && res.headers.location) {
          return fetch(res.headers.location).then(resolve).catch(reject);
        }
        if (res.statusCode !== 200) {
          return reject(new Error(`HTTP ${res.statusCode} for ${url}`));
        }
        resolve(res);
      })
      .on("error", reject);
  });
}

async function download(url, dest) {
  const res = await fetch(url);
  return new Promise((resolve, reject) => {
    const file = createWriteStream(dest);
    res.pipe(file);
    file.on("finish", () => file.close(resolve));
    file.on("error", reject);
  });
}

function extract(archive, destDir) {
  if (archive.endsWith(".tar.gz")) {
    execSync(`tar -xzf "${archive}" -C "${destDir}"`, { stdio: "ignore" });
  } else if (archive.endsWith(".zip")) {
    if (process.platform === "win32") {
      execSync(
        `powershell -Command "Expand-Archive -Path '${archive}' -DestinationPath '${destDir}' -Force"`,
        { stdio: "ignore" }
      );
    } else {
      execSync(`unzip -o "${archive}" -d "${destDir}"`, { stdio: "ignore" });
    }
  }
}

async function main() {
  const { platform, arch } = getPlatformArch();
  const pkg = require("../package.json");
  const version = pkg.version;

  const assetName = getAssetName(version, platform, arch);
  const url = `https://github.com/${REPO}/releases/download/v${version}/${assetName}`;

  const binDir = path.join(__dirname, "..", "bin");
  const tmpDir = path.join(__dirname, "..", ".tmp");

  fs.mkdirSync(binDir, { recursive: true });
  fs.mkdirSync(tmpDir, { recursive: true });

  const archivePath = path.join(tmpDir, assetName);

  console.log(`Downloading ${BINARY_NAME} v${version} for ${platform}/${arch}...`);

  try {
    await download(url, archivePath);
  } catch (err) {
    console.error(`Failed to download ${url}: ${err.message}`);
    process.exit(1);
  }

  extract(archivePath, tmpDir);

  const binaryExt = platform === "windows" ? ".exe" : "";
  const binarySource = path.join(tmpDir, `${BINARY_NAME}${binaryExt}`);
  const binaryDest = path.join(binDir, `.${BINARY_NAME}${binaryExt}`);

  fs.copyFileSync(binarySource, binaryDest);
  fs.chmodSync(binaryDest, 0o755);

  // Cleanup
  fs.rmSync(tmpDir, { recursive: true, force: true });

  console.log(`${BINARY_NAME} v${version} installed successfully.`);
}

main();
