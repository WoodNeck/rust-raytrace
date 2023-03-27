class RenderButton {
    private _el: HTMLButtonElement;
    private _callback: () => void;

    public constructor(el: HTMLButtonElement, callback: () => void) {
        this._el = el;
        this._callback = callback;

        el.innerText = 'Render!';
        el.disabled = false;
        el.addEventListener("click", this._onClick);
    }

    public enable() {
        this._el.disabled = false;
    }

    public disable() {
        this._el.disabled = true;
    }

    private _onClick = () => {
        this._callback();
    };
}

export default RenderButton;
