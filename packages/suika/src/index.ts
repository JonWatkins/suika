export { Component, Ctor } from "./Component";
export { mount } from "./render";
export { Reactive, ReactiveState, Listener, reactive } from "./Reactive";
export { Observable, Changes, Target } from "./observable";
export { ElementOptions, MapHas } from "./utils";

export {
  h,
  Fragment,
  vNode,
  vAttrs,
  vText,
  vElement,
  vFunction,
  vComponent,
  vFragment,
} from "./vdom";

export {
  isDef,
  isUndef,
  isEqual,
  isObject,
  makeMap,
  isReservedTag,
  isHTMLTag,
  isSVG,
  mergeClassNames,
  fixOptions,
} from "./utils";

declare global {
  namespace JSX {
    interface IntrinsicElements {
      // allow arbitrary elements
      // @ts-ignore suppress ts:2374 = Duplicate string index signature.
      [elemName: string]: any;
    }
  }
}
