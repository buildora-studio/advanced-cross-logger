'use strict'

const { platform, arch } = process

const PLATFORM_PACKAGES = {
  'darwin-arm64': '@cross-logger/node-darwin-arm64',
  'darwin-x64':   '@cross-logger/node-darwin-x64',
  'linux-x64':    '@cross-logger/node-linux-x64-gnu',
  'linux-arm64':  '@cross-logger/node-linux-arm64-gnu',
  'win32-x64':    '@cross-logger/node-win32-x64-msvc',
  'win32-arm64':  '@cross-logger/node-win32-arm64-msvc',
}

function load() {
  const key = `${platform}-${arch}`
  const pkg = PLATFORM_PACKAGES[key]

  if (pkg) {
    try { return require(pkg) } catch (_) {}
  }

  try { return require(`./index.${key}.node`) } catch (_) {}

  throw new Error(
    `cross-logger: no native binary found for ${key}.\n` +
    `Run 'npm run build' inside bindings/node to build from source.`
  )
}

const native = load()

const LogLevel = Object.freeze({
  OFF:   -1,
  SILLY:  0,
  DEBUG:  1,
  INFO:   2,
  WARN:   3,
  ERROR:  4,
  FATAL:  5,
})

module.exports = { ...native, LogLevel }
