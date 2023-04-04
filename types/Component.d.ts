import type { vNode } from "./vdom";
export declare abstract class Component {
  protected _uid: number;
  protected _el: HTMLElement | Text | null;
  protected _vNode: object | null;
  protected _mounted: boolean;
  protected state: object;
  protected attrs: object;
  protected _isSuika: boolean;
  constructor();
  private _update;
  _getDiff(): Function;
  _initVnode(attrs: object): object;
  _initState(): void;
  _setAttrs(attrs: object): void;
  _notifyMounted(el: HTMLElement): void;
  _unmount(): void;
  onMounted(): void;
  beforeUnmount(): void;
  onUpdated(): void;
  abstract render(): vNode;
}
