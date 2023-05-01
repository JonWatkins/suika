import { isDef } from "./utils";
import { isComponent, vNode, vAttrs } from "./vdom";
import { Component } from "./Component";

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
  applyAttributes(el, rootNode.attrs);

  if (rootNode.attrs.dangerouslySetHtml) {
    return dangerouslySetHtmlContent(el, rootNode.attrs);
  }

  rootNode.children.forEach((child: vNode) => {
    el.appendChild(render(child));
  });

  return el;
};

export const applyAttributes = (
  el: HTMLElement,
  attrs: vAttrs
): HTMLElement => {
  for (const attr in attrs) {
    if (attr !== "dangerouslySetHtml") {
      (el as any)[attr] = attrs[attr];
    }
  }

  return el;
};

export const dangerouslySetHtmlContent = (
  el: HTMLElement,
  attrs: vAttrs
): HTMLElement => {
  const { __html } = attrs.dangerouslySetHtml;
  const slotHTML = document.createRange().createContextualFragment(__html);
  el.innerHTML = "";
  el.appendChild(slotHTML);
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
