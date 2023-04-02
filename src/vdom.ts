import { Component } from "./Component";
import { isFunction, isString, isReservedTag, isDef } from "./utils";

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
    children,
  };
};

export const createText = (value) => ({
  kind: "text",
  value: value.toString(),
});

export const normalizeChildNodes = (childNodes) => {
  return childNodes
    .filter((i) => isDef(i))
    .map((childNode) =>
      isString(childNode) ? createText(childNode) : childNode
    );
};

export const h = (tag, attrs = {}, ...children) => {
  const normalized = normalizeChildNodes(children);

  if (isString(tag) && isReservedTag(tag)) {
    return createElement(tag, attrs, normalized);
  } else if (isFunction(tag)) {
    if (isFragment(tag)) {
      return createFragment(normalized);
    } else if (isComponent(tag)) {
      return createComponent(tag, attrs);
    } else {
      return createFunction(tag, attrs, normalized);
    }
  } else {
    return createText(tag);
  }
};
