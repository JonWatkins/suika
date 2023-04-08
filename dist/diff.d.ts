import { vNode, vAttrs } from "./vdom";
export type AttrsUpdater = {
    set: vAttrs;
    remove: string[];
};
export declare const diff: (oldVTree: vNode | null, newVTree: vNode | null) => Function;
export declare const diffAttrs: (oldAttrs: vAttrs, newAttrs: vAttrs) => (node: HTMLElement) => void;
export declare const diffChildNodes: (oldChildNodes: vNode[], newChildNodes: vNode[]) => (parent: HTMLElement) => HTMLElement;
export declare const unmountChildNodes: (oldTree: vNode, newTree: vNode | undefined) => void;
export declare const getToUnmount: (oldTree: vNode, newTree: vNode | undefined, toUnmount?: Array<any>) => any[];
export declare const getNewChildNode: (index: number, newTree: vNode | undefined) => vNode | undefined;
