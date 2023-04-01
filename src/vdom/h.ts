import { Component } from "../core/Component";
import { isFunction, isString, isReservedTag } from "../lib/utils";

export const Fragment = () => "FRAGMENT";

export type vDomAttrs = {
  [_: string]: string | number | boolean | Function;
};

export interface vDomElement {
  tag: string;
  attrs: object;
  children: vDomNode[];
  key?: string;
  kind: "element";
}

export interface vDomComponent {
  kind: "component";
  instance?: Component<any>;
  attrs: object;
  component: { new (): Component<any> };
  key?: string;
  tag: undefined;
  children: vDomNode[];
}

export interface vDomFunction {
  kind: "function";
  attrs: object;
  component: Function;
  tag: undefined;
  children: vDomNode[];
  key?: string;
}

export interface vDomText {
  value: string;
  key?: string;
  kind: "text";
  children: vDomNode[];
}

export interface vDomFragment {
  key?: string;
  kind: "fragment";
  attrs: object;
  children: vDomNode[];
  tag: string;
}

export type vDomNode =
  | vDomElement
  | vDomComponent
  | vDomFunction
  | vDomText
  | vDomFragment;

const mapChildNodes = (i): vDomNode => {
  if (isString(i)) return createText(i);
  return i;
};

const isComponent = (value: any): boolean => {
  return value.prototype instanceof Component;
};

const isFragment = (value: any): boolean => {
  return value === Fragment;
};

const createElement = (
  tag: string,
  attrs: vDomAttrs & { key?: string },
  children: vDomNode[]
): vDomElement => {
  if (!attrs) attrs = {};

  const key = attrs.key;
  delete attrs.key;

  return {
    tag,
    attrs,
    children,
    key,
    kind: "element",
  };
};

const createComponent = <P extends object>(
  component: { new (): Component<P> },
  attrs: P & { key?: string }
): vDomComponent => {
  const key = attrs.key;
  delete attrs.key;

  return {
    attrs,
    key,
    kind: "component",
    component,
    tag: undefined,
    children: [],
  };
};

const createFunction = (
  component: Function,
  attrs: vDomAttrs & { key?: string },
  children: vDomNode[]
): vDomFunction => {
  const key = attrs.key;
  delete attrs.key;

  return {
    attrs,
    key,
    kind: "function",
    component,
    tag: undefined,
    children,
  };
};

const createText = (value: string | number | boolean, key = ""): vDomText => ({
  key,
  kind: "text",
  value: value.toString(),
  children: [],
});

const createFragment = (
  tag: string,
  attrs: vDomAttrs & { key?: string },
  children: vDomNode[]
): vDomFragment => {
  if (!attrs) attrs = {};

  const key = attrs.key;
  delete attrs.key;

  return {
    key,
    tag,
    kind: "fragment",
    attrs,
    children,
  };
};

export const h = (tag, attrs, ...children: Array<vDomNode>): vDomNode => {
  if (isString(tag) && isReservedTag(tag)) {
    const childNodes: vDomNode[] = children.map(mapChildNodes);
    return createElement(tag, attrs, childNodes);
  } else if (isFunction(tag)) {
    if (isFragment(tag)) {
      return createFragment(tag(), attrs, children);
    } else if (isComponent(tag)) {
      return createComponent(tag, attrs || {});
    } else {
      return createFunction(tag, attrs || {}, children);
    }
  } else {
    return createText(tag);
  }
};
