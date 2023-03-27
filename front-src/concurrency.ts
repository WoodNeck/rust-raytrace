class Concurrency {
    private _el: HTMLInputElement;
    private _displayEl: HTMLElement;

    public get val() { return parseFloat(this._el.value); }
    public get max() { return parseFloat(this._el.max); }

    public constructor(el: HTMLInputElement, displayEl: HTMLElement) {
        this._el = el;
        this._displayEl = displayEl;

        el.min = "1";
        el.step = "1";
        el.max = navigator.hardwareConcurrency.toString();
        el.value = el.max;
        el.disabled = false;
        el.addEventListener("input", this._onInput);

        this._onInput();
    }

    private _onInput = () => {
        this._displayEl.innerText = 'Concurrency: ' + this._el.value;
    }
}

export default Concurrency;
