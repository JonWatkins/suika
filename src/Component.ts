import { observable } from "./observable";
import { diff } from "./diff";

let uid = 0;

export abstract class Component {
  protected _uid: number;
  protected _el: HTMLElement | Text | null;
  protected _vNode: object | null;
  protected _mounted: boolean;
  protected state: object;
  protected attrs: object;
  protected _isSuika = true;

  constructor() {
    this._uid = uid++;
  }

  private _update() {
    if (this._el) {
      const patch = this._getDiff();
      this._el = patch(this._el);
      this.onUpdated();
    }
  }

  protected _getDiff() {
    const vNode = this.render();
    const patch = diff(this._vNode, vNode);
    this._vNode = vNode;
    return patch;
  }

  protected _initVnode(attrs) {
    this._setAttrs(attrs);
    this._vNode = this.render();
    return this._vNode;
  }

  protected _initState() {
    this.state = observable(this.state || {}, this._update.bind(this));
  }

  protected _setAttrs(attrs) {
    this.attrs = attrs;
  }

  protected _notifyMounted(el) {
    this._el = el;
    this._mounted = true;
    this.onMounted();
  }

  protected _unmount() {
    this.beforeUnmount();
    this._el = null;
  }

  public onMounted() {}
  public beforeUnmount() {}
  public onUpdated() {}
  public abstract render();
}
