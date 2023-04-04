import { isDef } from "./utils";
import { isComponent } from "./vdom";
import { Component } from "./Component";

import type { vNode } from "./vdom";

export const render = (rootNode: vNode): HTMLElement | Text => {
  if (rootNode.kind === "text") {
    return document.createTextNode(rootNode.value);
  }

  if (rootNode.kind === "function") {
    const el = render(rootNode.component(rootNode.attrs, rootNode.children));
    return el;
  }

  if (rootNode.kind === "component") {
    if (rootNode.instance) {
      const el = render(rootNode.instance.render());
      rootNode.instance._notifyMounted(el as HTMLElement);
      return el;
    }

    rootNode.instance = new rootNode.component() as Component;
    rootNode.instance._initState();

    const vNode = rootNode.instance._initVnode(rootNode.attrs);
    const el = render(vNode);

    rootNode.instance._notifyMounted(el as HTMLElement);
    return el;
  }

  const el = document.createElement(rootNode.tag);

  for (const attr in rootNode.attrs) {
    (el as any)[attr] = rootNode.attrs[attr];
  }

  rootNode.children.forEach((child) => {
    el.appendChild(render(child));
  });

  return el;
};

export const mount = (component: new () => Component, root: HTMLElement) => {
  if (!isDef(component) || !isComponent(component)) {
    throw new Error("Must pass a component to mount");
  }

  if (!isDef(component) || !(root instanceof HTMLElement)) {
    throw new Error("Must pass a dom node to mount");
  }

  const instance = new component();
  instance._initState();

  const el = render(instance._initVnode({})) as HTMLElement;

  instance._notifyMounted(el);
  root.replaceWith(el);

  return instance;
};
