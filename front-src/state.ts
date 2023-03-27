class RenderState {
    private timingVal: HTMLElement;
    private ctx: CanvasRenderingContext2D;
    private onEnd: () => void;
    private wasm: any;
    private start: number;
    private running: boolean;
    private counter: number;
    private interval: number;

    constructor({
        wasm,
        timingVal,
        onEnd,
        canvas
    }) {
      this.start = performance.now();
      this.wasm = wasm;
      this.timingVal = timingVal;
      this.onEnd = onEnd;
      this.running = true;
      this.counter = 1;
      this.ctx = canvas.getContext("2d");

      this.interval = window.setInterval(() => this.updateTimer(true), 100);

      wasm.promise()
        .then(data => {
          this.updateTimer(false);
          this.updateImage(data);
          this.stop();
        })
        .catch(console.error);
    }

    updateTimer(updateImage: boolean) {
      const dur = performance.now() - this.start;
      this.timingVal.innerText = `${dur}ms`;
      this.counter += 1;

      if (updateImage && this.wasm && this.counter % 3 == 0)
        this.updateImage(this.wasm.imageSoFar());
    }

    updateImage(data: ImageData) {

      console.log(data);

      this.ctx.putImageData(data, 0, 0);
    }

    stop() {
      if (!this.running)
        return;
      console.timeEnd('render');
      this.running = false;
      this.wasm = null;
      clearInterval(this.interval);
      this.onEnd();
    }
}

export default RenderState;
