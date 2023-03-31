import type { vDomNode } from "./h";

export function render(rootNode: vDomNode): HTMLElement | Text {
  if (rootNode.kind === "text") {
    return document.createTextNode(rootNode.value);
  }

  if (rootNode.kind === "component") {
    if (rootNode.instance) {
      const el = render(rootNode.instance.render());
      rootNode.instance._notifyMounted(el as HTMLElement);
      return el;
    }

    rootNode.instance = new rootNode.component();
    rootNode.instance._initState();

    const vNode = rootNode.instance._initProps(rootNode.attrs);
    const el = render(vNode);

    rootNode.instance._notifyMounted(el as HTMLElement);
    return el;
  }

  const el = document.createElement(rootNode.tag);

  for (const attr in rootNode.attrs) {
    (el as any)[attr] = rootNode.attrs[attr];
  }

  renderChildNodes(el, rootNode.children);
  return el;
}

function renderChildNodes(el, childNodes) {
  childNodes.forEach((child) => {
    el.appendChild(render(child));
  });
}
