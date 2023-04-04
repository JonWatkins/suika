import { vNode, vAttrs } from "./vdom";
export type AttrsUpdater = {
    set: vAttrs;
    remove: string[];
};
export declare function diff(oldVTree: vNode | null, newVTree: vNode | null): Function;
export declare function diffAttrs(oldAttrs: vAttrs, newAttrs: vAttrs): (node: HTMLElement) => void;
export declare function diffChildNodes(oldChildNodes: vNode[], newChildNodes: vNode[]): (parent: HTMLElement) => HTMLElement;
export declare function unmountChildNodes(oldNode: vNode, newNode: vNode | null): void;
export declare const zip: (xs: Array<any>, ys: Array<any>) => any[][];
