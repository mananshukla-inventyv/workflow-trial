/* tslint:disable */
/* eslint-disable */
/* prettier-ignore */

/* auto-generated by NAPI-RS */

const { existsSync, readFileSync } = require('fs')
const { join } = require('path')

const { platform, arch } = process

let nativeBinding = null
let localFileExisted = false
let loadError = null

function isMusl() {
  // For Node 10
  if (!process.report || typeof process.report.getReport !== 'function') {
    try {
      const lddPath = require('child_process').execSync('which ldd').toString().trim()
      return readFileSync(lddPath, 'utf8').includes('musl')
    } catch (e) {
      return true
    }
  } else {
    const { glibcVersionRuntime } = process.report.getReport().header
    return !glibcVersionRuntime
  }
}

switch (platform) {
  case 'android':
    switch (arch) {
      case 'arm64':
        localFileExisted = existsSync(join(__dirname, 'workflow-trial.android-arm64.node'))
        try {
          if (localFileExisted) {
            nativeBinding = require('./workflow-trial.android-arm64.node')
          } else {
            nativeBinding = require('workflow-test-android-arm64')
          }
        } catch (e) {
          loadError = e
        }
        break
      case 'arm':
        localFileExisted = existsSync(join(__dirname, 'workflow-trial.android-arm-eabi.node'))
        try {
          if (localFileExisted) {
            nativeBinding = require('./workflow-trial.android-arm-eabi.node')
          } else {
            nativeBinding = require('workflow-test-android-arm-eabi')
          }
        } catch (e) {
          loadError = e
        }
        break
      default:
        throw new Error(`Unsupported architecture on Android ${arch}`)
    }
    break
  case 'win32':
    switch (arch) {
      case 'x64':
        localFileExisted = existsSync(join(__dirname, 'workflow-trial.win32-x64-msvc.node'))
        try {
          if (localFileExisted) {
            nativeBinding = require('./workflow-trial.win32-x64-msvc.node')
          } else {
            nativeBinding = require('workflow-test-win32-x64-msvc')
          }
        } catch (e) {
          loadError = e
        }
        break
      case 'ia32':
        localFileExisted = existsSync(join(__dirname, 'workflow-trial.win32-ia32-msvc.node'))
        try {
          if (localFileExisted) {
            nativeBinding = require('./workflow-trial.win32-ia32-msvc.node')
          } else {
            nativeBinding = require('workflow-test-win32-ia32-msvc')
          }
        } catch (e) {
          loadError = e
        }
        break
      case 'arm64':
        localFileExisted = existsSync(join(__dirname, 'workflow-trial.win32-arm64-msvc.node'))
        try {
          if (localFileExisted) {
            nativeBinding = require('./workflow-trial.win32-arm64-msvc.node')
          } else {
            nativeBinding = require('workflow-test-win32-arm64-msvc')
          }
        } catch (e) {
          loadError = e
        }
        break
      default:
        throw new Error(`Unsupported architecture on Windows: ${arch}`)
    }
    break
  case 'darwin':
    localFileExisted = existsSync(join(__dirname, 'workflow-trial.darwin-universal.node'))
    try {
      if (localFileExisted) {
        nativeBinding = require('./workflow-trial.darwin-universal.node')
      } else {
        nativeBinding = require('workflow-test-darwin-universal')
      }
      break
    } catch {}
    switch (arch) {
      case 'x64':
        localFileExisted = existsSync(join(__dirname, 'workflow-trial.darwin-x64.node'))
        try {
          if (localFileExisted) {
            nativeBinding = require('./workflow-trial.darwin-x64.node')
          } else {
            nativeBinding = require('workflow-test-darwin-x64')
          }
        } catch (e) {
          loadError = e
        }
        break
      case 'arm64':
        localFileExisted = existsSync(join(__dirname, 'workflow-trial.darwin-arm64.node'))
        try {
          if (localFileExisted) {
            nativeBinding = require('./workflow-trial.darwin-arm64.node')
          } else {
            nativeBinding = require('workflow-test-darwin-arm64')
          }
        } catch (e) {
          loadError = e
        }
        break
      default:
        throw new Error(`Unsupported architecture on macOS: ${arch}`)
    }
    break
  case 'freebsd':
    if (arch !== 'x64') {
      throw new Error(`Unsupported architecture on FreeBSD: ${arch}`)
    }
    localFileExisted = existsSync(join(__dirname, 'workflow-trial.freebsd-x64.node'))
    try {
      if (localFileExisted) {
        nativeBinding = require('./workflow-trial.freebsd-x64.node')
      } else {
        nativeBinding = require('workflow-test-freebsd-x64')
      }
    } catch (e) {
      loadError = e
    }
    break
  case 'linux':
    switch (arch) {
      case 'x64':
        if (isMusl()) {
          localFileExisted = existsSync(join(__dirname, 'workflow-trial.linux-x64-musl.node'))
          try {
            if (localFileExisted) {
              nativeBinding = require('./workflow-trial.linux-x64-musl.node')
            } else {
              nativeBinding = require('workflow-test-linux-x64-musl')
            }
          } catch (e) {
            loadError = e
          }
        } else {
          localFileExisted = existsSync(join(__dirname, 'workflow-trial.linux-x64-gnu.node'))
          try {
            if (localFileExisted) {
              nativeBinding = require('./workflow-trial.linux-x64-gnu.node')
            } else {
              nativeBinding = require('workflow-test-linux-x64-gnu')
            }
          } catch (e) {
            loadError = e
          }
        }
        break
      case 'arm64':
        if (isMusl()) {
          localFileExisted = existsSync(join(__dirname, 'workflow-trial.linux-arm64-musl.node'))
          try {
            if (localFileExisted) {
              nativeBinding = require('./workflow-trial.linux-arm64-musl.node')
            } else {
              nativeBinding = require('workflow-test-linux-arm64-musl')
            }
          } catch (e) {
            loadError = e
          }
        } else {
          localFileExisted = existsSync(join(__dirname, 'workflow-trial.linux-arm64-gnu.node'))
          try {
            if (localFileExisted) {
              nativeBinding = require('./workflow-trial.linux-arm64-gnu.node')
            } else {
              nativeBinding = require('workflow-test-linux-arm64-gnu')
            }
          } catch (e) {
            loadError = e
          }
        }
        break
      case 'arm':
        localFileExisted = existsSync(join(__dirname, 'workflow-trial.linux-arm-gnueabihf.node'))
        try {
          if (localFileExisted) {
            nativeBinding = require('./workflow-trial.linux-arm-gnueabihf.node')
          } else {
            nativeBinding = require('workflow-test-linux-arm-gnueabihf')
          }
        } catch (e) {
          loadError = e
        }
        break
      default:
        throw new Error(`Unsupported architecture on Linux: ${arch}`)
    }
    break
  default:
    throw new Error(`Unsupported OS: ${platform}, architecture: ${arch}`)
}

if (!nativeBinding) {
  if (loadError) {
    throw loadError
  }
  throw new Error(`Failed to load native binding`)
}

const {
  getNextKey,
  initClient,
  startLogger,
  getDocument,
  addDocument,
  replaceDocument,
  getBatchUsingScan,
  getBatch,
  deleteSingleRecord,
} = nativeBinding

module.exports.getNextKey = getNextKey
module.exports.initClient = initClient
module.exports.startLogger = startLogger
module.exports.getDocument = getDocument
module.exports.addDocument = addDocument
module.exports.replaceDocument = replaceDocument
module.exports.getBatchUsingScan = getBatchUsingScan
module.exports.getBatch = getBatch
module.exports.deleteSingleRecord = deleteSingleRecord
