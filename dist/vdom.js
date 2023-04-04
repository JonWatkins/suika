import { Component } from "./Component";
import { isReservedTag, isDef } from "./utils";
export const Fragment = () => "fragment";
export const isComponent = (value) => {
  return value.prototype instanceof Component;
};
export const isFragment = (value) => {
  return value === Fragment && value() === value();
};
export const createElement = (tag, attrs, children) => {
  return {
    kind: "element",
    tag,
    attrs,
    children,
  };
};
export const createComponent = (component, attrs) => {
  return {
    kind: "component",
    instance: undefined,
    attrs,
    component,
  };
};
export const createFunction = (component, attrs, children) => {
  return {
    kind: "function",
    attrs,
    component,
    children,
  };
};
export const createFragment = (children) => {
  return {
    kind: "fragment",
    tag: "fragment",
    children,
    attrs: {},
  };
};
export const createText = (value) => ({
  kind: "text",
  value: value.toString(),
});
export const normalizeChildNodes = (childNodes) => {
  return childNodes
    .filter((i) => isDef(i))
    .map((childNode) => {
      let res;
      if (typeof childNode === "string") {
        res = createText(childNode);
      } else {
        res = childNode;
      }
      return res;
    });
};
export const h = (tag, attrs = {}, ...children) => {
  const normalized = normalizeChildNodes(children);
  if (typeof tag === "string" && isReservedTag(tag)) {
    return createElement(tag, attrs, normalized);
  }
  if (typeof tag === "function") {
    if (isFragment(tag)) {
      return createFragment(normalized);
    } else if (isComponent(tag)) {
      return createComponent(tag, attrs);
    } else {
      return createFunction(tag, attrs, normalized);
    }
  }
  return createText(tag);
};
//# sourceMappingURL=vdom.js.map
