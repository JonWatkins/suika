import type { vNode, vAttrs } from "./vdom";
export declare abstract class Component {
    _el: HTMLElement | Text | null;
    _vNode: object | null;
    _mounted: boolean;
    state: object;
    attrs: vAttrs;
    _isSuika: boolean;
    _uid: number;
    constructor();
    private _update;
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
