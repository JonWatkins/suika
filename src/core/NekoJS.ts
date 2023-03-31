import { Context } from "./Context";

export abstract class NekoJS extends Context {
  constructor() {
    super();
  }

  protected _mount(el: HTMLElement): void {
    this._el = el;
    this._mounted = true;
    this._initState();
    this._update();
  }
}
