import { Component, Ctor } from "./Component";
export type vAttrs = {
    [_: string]: any;
};
export interface vText {
    kind: "text";
    value: string;
}
export interface vElement {
    kind: "element";
    children: vNode[];
    tag: string;
    attrs: vAttrs;
}
export interface vFragment {
    kind: "fragment";
    tag: "fragment";
    children: vNode[];
    attrs: vAttrs;
}
export interface vFunction {
    kind: "function";
    children: vNode[];
    component: Function;
    attrs: vAttrs;
}
export interface vComponent {
    kind: "component";
    component: Ctor;
    instance?: Component;
    attrs: vAttrs;
}
export type vNode = vText | vElement | vFunction | vComponent | vFragment;
export declare const Fragment: () => string;
export declare const isComponent: (value: any) => boolean;
export declare const isFragment: (value: Function) => boolean;
export declare const createElement: (tag: string, attrs: vAttrs, children: vNode[]) => vElement;
export declare const createComponent: (component: Ctor, attrs: vAttrs) => vComponent;
export declare const createFunction: (component: Function, attrs: vAttrs, children: vNode[]) => vFunction;
export declare const createFragment: (children: vNode[]) => vFragment;
export declare const createText: (value: string) => vText;
export declare const normalizeChildNodes: (childNodes: Array<vNode | string>) => vNode[];
export declare const h: (tag: string | Ctor | Function, attrs?: vAttrs, ...children: Array<vNode | string>) => vNode;
