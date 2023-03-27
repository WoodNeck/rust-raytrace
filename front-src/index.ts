import RenderButton from "./button";
import Concurrency from "./concurrency";
import loadWasm from "./load-wasm";
import RenderState from "./state";

const buttonEl = document.getElementById('render') as HTMLButtonElement;
const canvas = document.getElementById('canvas') as HTMLCanvasElement;
const concurrencyEl = document.getElementById('concurrency') as HTMLInputElement;
const concurrencyAmt = document.getElementById('concurrency-amt') as HTMLElement;
const timingVal = document.getElementById('timing-val');

const main = async () => {
  // First up, but try to do feature detection to provide better error messages
  await loadWasm();

  const { Raycaster, WorkerPool } = wasm_bindgen;

  canvas.width = canvas.clientWidth;
  canvas.height = canvas.clientHeight;

  // The maximal concurrency of our web worker pool is `hardwareConcurrency`,
  // so set that up here and this ideally is the only location we create web
  // workers.
  const concurrency = new Concurrency(concurrencyEl, concurrencyAmt);
  const pool = new WorkerPool(concurrency.max);
  const raycaster = new Raycaster(canvas.clientWidth, canvas.clientHeight);

  const btn = new RenderButton(buttonEl, () => {
    console.time("render");
    return new RenderState({
      wasm: raycaster.render(concurrency.val, pool),
      timingVal,
      canvas,
      onEnd: () => {
        btn.enable();
        console.timeEnd("render");
      }
    });
  });

};

main();

