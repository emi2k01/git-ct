const { Binary } = require("@emi2k01/binary-install");
const os = require("os");
const cTable = require("console.table");

const VERSION = "0.0.4";

const error = (msg) => {
  console.error(msg);
  process.exit(1);
};

const { version, name, repository } = require("./package.json");

const supportedPlatforms = [
  {
    TYPE: "Windows_NT",
    ARCHITECTURE: "x64",
    RUST_TARGET: "x86_64-pc-windows-msvc",
    BINARY_NAME: "git-ct.exe",
  },
  {
    TYPE: "Linux",
    ARCHITECTURE: "x64",
    RUST_TARGET: "x86_64-unknown-linux-musl",
    BINARY_NAME: "git-ct",
  },
  {
    TYPE: "Darwin",
    ARCHITECTURE: "x64",
    RUST_TARGET: "x86_64-apple-darwin",
    BINARY_NAME: "git-ct",
  },
];

const getPlatformMetadata = () => {
  const type = os.type();
  const architecture = os.arch();

  for (let supportedPlatform of supportedPlatforms) {
    if (
      type === supportedPlatform.TYPE &&
      architecture === supportedPlatform.ARCHITECTURE
    ) {
      return supportedPlatform;
    }
  }

  error(
    `Platform with type "${type}" and architecture "${architecture}" is not supported by ${name}.\nYour system must be one of the following:\n\n${cTable.getTable(
      supportedPlatforms
    )}`
  );
};

const getBinary = () => {
  const platformMetadata = getPlatformMetadata();
  // the url for this binary is constructed from values in `package.json`
  // https://github.com/EverlastingBugstopper/binary-install/releases/download/v1.0.0/binary-install-example-v1.0.0-x86_64-apple-darwin.tar.gz
  const url = `${repository.url}/releases/download/v${VERSION}/${name}_v${VERSION}_${platformMetadata.RUST_TARGET}.tar.gz`;
  return new Binary(platformMetadata.BINARY_NAME, url);
};

const run = () => {
  const binary = getBinary();
  binary.run();
};

const install = () => {
  const binary = getBinary();
  binary.install();
};

const uninstall = () => {
  const binary = getBinary();
  binary.uninstall();
};

module.exports = {
  install,
  run,
  uninstall,
};
