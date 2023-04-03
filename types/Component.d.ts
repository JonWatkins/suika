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
    protected _getDiff(): any;
    protected _initVnode(attrs: any): object | null;
    protected _initState(): void;
    protected _setAttrs(attrs: any): void;
    protected _notifyMounted(el: any): void;
    protected _unmount(): void;
    onMounted(): void;
    beforeUnmount(): void;
    onUpdated(): void;
    abstract render(): any;
}
