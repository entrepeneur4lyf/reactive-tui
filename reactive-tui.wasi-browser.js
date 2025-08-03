import {
  createOnMessage as __wasmCreateOnMessageForFsProxy,
  getDefaultContext as __emnapiGetDefaultContext,
  instantiateNapiModuleSync as __emnapiInstantiateNapiModuleSync,
  WASI as __WASI,
} from '@napi-rs/wasm-runtime'



const __wasi = new __WASI({
  version: 'preview1',
})

const __wasmUrl = new URL('./reactive-tui.wasm32-wasi.wasm', import.meta.url).href
const __emnapiContext = __emnapiGetDefaultContext()


const __sharedMemory = new WebAssembly.Memory({
  initial: 4000,
  maximum: 65536,
  shared: true,
})

const __wasmFile = await fetch(__wasmUrl).then((res) => res.arrayBuffer())

const {
  instance: __napiInstance,
  module: __wasiModule,
  napiModule: __napiModule,
} = __emnapiInstantiateNapiModuleSync(__wasmFile, {
  context: __emnapiContext,
  asyncWorkPoolSize: 4,
  wasi: __wasi,
  onCreateWorker() {
    const worker = new Worker(new URL('./wasi-worker-browser.mjs', import.meta.url), {
      type: 'module',
    })

    return worker
  },
  overwriteImports(importObject) {
    importObject.env = {
      ...importObject.env,
      ...importObject.napi,
      ...importObject.emnapi,
      memory: __sharedMemory,
    }
    return importObject
  },
  beforeInit({ instance }) {
    for (const name of Object.keys(instance.exports)) {
      if (name.startsWith('__napi_register__')) {
        instance.exports[name]()
      }
    }
  },
})
export default __napiModule.exports
export const Actions = __napiModule.exports.Actions
export const EnhancedFfiTypes = __napiModule.exports.EnhancedFfiTypes
export const EnhancedFFITypes = __napiModule.exports.EnhancedFFITypes
export const JsColorDefinition = __napiModule.exports.JsColorDefinition
export const JsColorTheme = __napiModule.exports.JsColorTheme
export const JsElement = __napiModule.exports.JsElement
export const JsReactiveState = __napiModule.exports.JsReactiveState
export const JsToast = __napiModule.exports.JsToast
export const JsToastManager = __napiModule.exports.JsToastManager
export const JsTuiApp = __napiModule.exports.JsTuiApp
export const TuiUtils = __napiModule.exports.TuiUtils
export const getVersion = __napiModule.exports.getVersion
export const init = __napiModule.exports.init
export const initTui = __napiModule.exports.initTui
