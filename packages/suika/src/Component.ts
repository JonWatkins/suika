import { reactive, ReactiveState } from "./Reactive";
import { diff } from "./diff";
import type { vNode, vAttrs } from "./vdom";

export type Ctor = new () => Component;

let uid = 0;

export abstract class Component {
  public _uid: number;
  public _el: HTMLElement | Text | null;
  public _vNode: vNode | null;
  public _mounted: boolean;
  public _isSuika: boolean;
  public _onChange = () => this._update();
  public attrs: vAttrs;
  public state;

  constructor() {
    this._uid = uid++;
    this._el = null;
    this._vNode = null;
    this._mounted = false;
    this._isSuika = true;
    this.state = {} as ReactiveState;
    this.attrs = {};
  }

  public _update(): void {
    if (this._el) {
      const patch = this._getDiff();
      this._el = patch(this._el as HTMLElement);
      this.onUpdated();
    }
  }

  public _getDiff(): Function {
    const vNode = this.render();
    const patch = diff(this._vNode as vNode, vNode);
    this._vNode = vNode;
    return patch;
  }

  public _initVnode(attrs: vAttrs): vNode {
    this._setAttrs(attrs);
    const vNode = this.render();
    this._vNode = vNode;
    return vNode;
  }

  public _initState(): void {
    if (!this.state._isReactive) {
      this.state = reactive(this.state);
    }

    this.state.addListener(this._onChange);
  }

  public _setAttrs(attrs: vAttrs) {
    this.attrs = attrs;
  }

  public _notifyMounted(el: HTMLElement): void {
    if (!this._mounted) {
      this._el = el;
      this._mounted = true;
      this.onMounted();
    }
  }

  public _unmount(): void {
    this.state.removeListener(this._onChange);
    this.beforeUnmount();
    this._el = null;
  }

  public onMounted(): void {}
  public beforeUnmount(): void {}
  public onUpdated(): void {}
  public abstract render(): vNode;
}
