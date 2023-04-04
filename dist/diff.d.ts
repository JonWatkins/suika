import { vNode, vAttrs, vComponent } from "./vdom";
export type AttrsUpdater = {
    set: vAttrs;
    remove: string[];
};
export declare const diff: (oldVTree: vNode | null, newVTree: vNode | null) => Function;
export declare const diffAttrs: (oldAttrs: vAttrs, newAttrs: vAttrs) => (node: HTMLElement) => void;
export declare const diffChildNodes: (oldChildNodes: vNode[], newChildNodes: vNode[]) => (parent: HTMLElement) => HTMLElement;
export declare const unmountChildNodes: (oldTree: vNode, newTree: vNode | null, toUnmount?: Array<vComponent>) => void;
export declare const zip: (xs: Array<any>, ys: Array<any>) => any[][];
