declare namespace wasm_bindgen {
	/* tslint:disable */
	/* eslint-disable */
	/**
	* Entry point invoked by `worker.js`, a bit of a hack but see the "TODO" above
	* about `worker.js` in general.
	* @param {number} ptr
	*/
	export function child_entry_point(ptr: number): void;
	/**
	*/
	export class Raycaster {
	  free(): void;
	/**
	* @param {number} width
	* @param {number} height
	*/
	  constructor(width: number, height: number);
	/**
	* @param {number} concurrency
	* @param {WorkerPool} pool
	* @returns {RenderResult}
	*/
	  render(concurrency: number, pool: WorkerPool): RenderResult;
	/**
	*/
	  height: number;
	/**
	*/
	  width: number;
	}
	/**
	*/
	export class RenderResult {
	  free(): void;
	/**
	* Returns the JS promise object which resolves when the render is complete
	* @returns {Promise<any>}
	*/
	  promise(): Promise<any>;
	/**
	* Return a progressive rendering of the image so far
	* @returns {any}
	*/
	  imageSoFar(): any;
	}
	/**
	*/
	export class WorkerPool {
	  free(): void;
	/**
	* Creates a new `WorkerPool` which immediately creates `initial` workers.
	*
	* The pool created here can be used over a long period of time, and it
	* will be initially primed with `initial` workers. Currently workers are
	* never released or gc'd until the whole pool is destroyed.
	*
	* # Errors
	*
	* Returns any error that may happen while a JS web worker is created and a
	* message is sent to it.
	* @param {number} initial
	*/
	  constructor(initial: number);
	}
	
}

declare type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

declare interface InitOutput {
  readonly __wbg_workerpool_free: (a: number) => void;
  readonly workerpool_new: (a: number, b: number) => void;
  readonly child_entry_point: (a: number, b: number) => void;
  readonly __wbg_raycaster_free: (a: number) => void;
  readonly __wbg_get_raycaster_width: (a: number) => number;
  readonly __wbg_set_raycaster_width: (a: number, b: number) => void;
  readonly __wbg_get_raycaster_height: (a: number) => number;
  readonly __wbg_set_raycaster_height: (a: number, b: number) => void;
  readonly raycaster_new: (a: number, b: number, c: number) => void;
  readonly raycaster_render: (a: number, b: number, c: number, d: number) => void;
  readonly __wbg_renderresult_free: (a: number) => void;
  readonly renderresult_promise: (a: number) => number;
  readonly renderresult_imageSoFar: (a: number) => number;
  readonly memory: WebAssembly.Memory;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_export_3: WebAssembly.Table;
  readonly _dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__he3dcd9e3c2d6d5e0: (a: number, b: number, c: number) => void;
  readonly _dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h5e72e0d5f2cab0e7: (a: number, b: number, c: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __wbindgen_free: (a: number, b: number) => void;
  readonly wasm_bindgen__convert__closures__invoke2_mut__h0d9a184cb72edd85: (a: number, b: number, c: number, d: number) => void;
  readonly __wbindgen_thread_destroy: (a: number, b: number) => void;
  readonly __wbindgen_start: () => void;
}

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
* @param {WebAssembly.Memory} maybe_memory
*
* @returns {Promise<InitOutput>}
*/
declare function wasm_bindgen (module_or_path?: InitInput | Promise<InitInput>, maybe_memory?: WebAssembly.Memory): Promise<InitOutput>;
