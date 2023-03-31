import { vDomNode } from "../vdom/h";
import { observable } from "../lib/observable";
import { diff, vDomUpdater } from "../vdom/diff";
import { patch } from "../vdom/patch";

let uid = 0;

export abstract class Context {
  protected _uid: number;
  protected _el: HTMLElement | Text | null;
  protected _vNode: vDomNode | null;
  protected _mounted: boolean;
  protected _isNekoJS: boolean;
  protected state: object;

  constructor() {
    this._uid = uid++;
  }

  protected _getDiff(): vDomUpdater {
    const vNode = this.render();
    const updater = diff(this._vNode, vNode);

    if (updater.kind === "replace") {
      updater.callback = (el) => (this._el = el);
    }

    this._vNode = vNode;

    return updater;
  }

  protected _update(): void {
    if (!this._el) return;
    patch(this._el, this._getDiff());
    this._notifyUpdated();
  }

  public _initState() {
    this.state = observable(this.state || {}, this._update.bind(this));
  }

  public _unmount() {
    this.beforeUnmount();
    this._el = null;
  }

  public _notifyMounted(el: HTMLElement | Text) {
    this._el = el;
    setTimeout(() => this.onMounted());
  }

  public _notifyUpdated() {
    setTimeout(() => this.onUpdated());
  }

  public onMounted() {}
  public onUpdated() {}
  public beforeUnmount() {}

  public abstract render(): vDomNode;
}
