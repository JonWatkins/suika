import { string } from "yargs";
import { Component } from "./Component";
import { isReservedTag, isDef } from "./utils";

export type vAttrs = object;

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
  children: vNode[];
}

export interface vFunction {
  kind: "function";
  children: vNode[];
  component: Function;
  attrs: vAttrs;
}

export interface vComponent {
  kind: "component";
  component: Function;
  instance?: Component;
  attrs: vAttrs;
}

export type vNode = vText | vElement | vFragment | vFunction | vComponent;

export const Fragment = (): string => "fragment";

export const isComponent = (value: any): boolean => {
  return value.prototype instanceof Component;
};

export const isFragment = (value: Function): boolean => {
  return value === Fragment && value() === value();
};

export const createElement = (
  tag: string,
  attrs: vAttrs,
  children: vNode[]
): vElement => {
  return {
    kind: "element",
    tag,
    attrs,
    children,
  };
};

export const createComponent = (
  component: Function,
  attrs: vAttrs
): vComponent => {
  return {
    kind: "component",
    instance: undefined,
    attrs,
    component,
  };
};

export const createFunction = (
  component: Function,
  attrs: vAttrs,
  children: vNode[]
): vFunction => {
  return {
    kind: "function",
    attrs,
    component,
    children,
  };
};

export const createFragment = (children: vNode[]): vFragment => {
  return {
    kind: "fragment",
    children,
  };
};

export const createText = (value: string): vText => ({
  kind: "text",
  value: value.toString(),
});

export const normalizeChildNodes = (
  childNodes: Array<vNode | string>
): vNode[] => {
  return childNodes
    .filter((i) => isDef(i))
    .map((childNode: vNode | string): vNode => {
      let res: vNode;
      if (typeof childNode === "string") {
        res = createText(childNode as string);
      } else {
        res = childNode;
      }
      return res;
    });
};

export const h = (
  tag: string | Function,
  attrs: vAttrs = {},
  ...children: Array<vNode | string>
): vNode => {
  const normalized = normalizeChildNodes(children);

  if (typeof tag === "string" && isReservedTag(tag)) {
    return createElement(tag, attrs, normalized);
  } else if (typeof tag === "function") {
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
