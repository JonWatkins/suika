export { Component } from "./Component";
export { h, Fragment } from "./vdom";
export { mount } from "./render";

export {
  isDef,
  isUndef,
  isEqual,
  isObject,
  makeMap,
  isReservedTag,
  isHTMLTag,
  isSVG,
} from "./utils";

declare global {
  namespace JSX {
    interface IntrinsicElements {
      [elemName: string]: any;
    }
  }
}
