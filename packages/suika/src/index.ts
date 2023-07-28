export { createElement } from "./vdom";
export { useState, useEffect } from "./hooks";
export { render } from "./render";

declare global {
  namespace JSX {
    interface IntrinsicElements {
      // allow arbitrary elements
      // @ts-ignore suppress ts:2374 = Duplicate string index signature.
      // eslint-disable-next-line
      [elemName: string]: any;
    }
  }
}
