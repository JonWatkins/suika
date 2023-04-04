import { Observable } from "./observable";
import type { vNode, vAttrs } from "./vdom";
export type Ctor = new () => Component;
export interface BaseState {
    [_: string]: any;
}
export declare abstract class Component {
    _el: HTMLElement | Text | null;
    _vNode: vNode | null;
    _mounted: boolean;
    state: Observable | BaseState;
    attrs: vAttrs;
    _isSuika: boolean;
    _uid: number;
    constructor();
    _update(): void;
    _getDiff(): Function;
    _initVnode(attrs: vAttrs): vNode;
    _initState(): void;
    _setAttrs(attrs: vAttrs): void;
    _notifyMounted(el: HTMLElement): void;
    _unmount(): void;
    onMounted(): void;
    beforeUnmount(): void;
    onUpdated(): void;
    abstract render(): vNode;
}
