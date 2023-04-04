import { Component } from "./Component";
import type { vNode } from "./vdom";
export declare const render: (rootNode: vNode) => HTMLElement | Text;
export declare const mount: (component: Component, root: HTMLElement) => any;
