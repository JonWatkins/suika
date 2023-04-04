import { observable } from "./observable";
import { diff } from "./diff";
import type { vNode } from "./vdom";

let uid = 0;

export abstract class Component {
  protected _uid: number;
  protected _el: HTMLElement | Text | null;
  protected _vNode: object | null;
  protected _mounted: boolean;
  protected state: object;
  protected attrs: object;
  protected _isSuika: boolean;

  constructor() {
    this._uid = uid++;
    this._el = null;
    this._vNode = null;
    this._mounted = false;
    this._isSuika = true;
    this.state = {};
    this.attrs = {};
  }

  private _update() {
    if (this._el) {
      const patch = this._getDiff();
      this._el = patch(this._el);
      this.onUpdated();
    }
  }

  public _getDiff() {
    const vNode = this.render();
    const patch = diff(this._vNode, vNode);
    this._vNode = vNode;
    return patch;
  }

  public _initVnode(attrs: object) {
    this._setAttrs(attrs);
    this._vNode = this.render();
    return this._vNode;
  }

  public _initState() {
    this.state = observable(this.state, this._update.bind(this));
  }

  public _setAttrs(attrs: object) {
    this.attrs = attrs;
  }

  public _notifyMounted(el: HTMLElement) {
    this._el = el;
    this._mounted = true;
    this.onMounted();
  }

  public _unmount() {
    this.beforeUnmount();
    this._el = null;
  }

  public onMounted() {}
  public beforeUnmount() {}
  public onUpdated() {}
  public abstract render(): vNode;
}
