export { Component } from "./Component";
export { h, Fragment } from "./vdom";
export { mount } from "./render";
export * as utils from "./utils";
declare global {
    namespace JSX {
        interface IntrinsicElements {
            [elemName: string]: HTMLElement;
        }
    }
}
