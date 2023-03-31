import { ChildUpdater, vDomUpdater } from "./diff";
import { render } from "./render";

export function patch(
  el: HTMLElement | Text,
  patches: vDomUpdater
): HTMLElement | Text | DocumentFragment {
  if (patches.kind === "skip") return el;

  if (patches.kind == "replace") {
    const newEl = render(patches.newNode);
    el?.replaceWith(newEl);
    if (patches.callback) patches.callback(newEl);
    return newEl;
  }

  for (const att in patches.attrs.remove) {
    el.removeAttribute(att);
  }

  for (const att in patches.attrs.set) {
    (el as any)[att] = patches.attrs.set[att];
  }

  patchChildren(el, patches.childeren);

  return el;
}

const patchChildren = (el: HTMLElement | Text, operations: ChildUpdater[]) => {
  let offset = 0;
  for (let i = 0; i < operations.length; i++) {
    const childUpdater = operations[i];

    if (childUpdater.kind == "skip") continue;

    if (childUpdater.kind == "insert") {
      if (el.childNodes[i + offset - 1])
        el.childNodes[i + offset - 1].after(render(childUpdater.node));
      else el.appendChild(render(childUpdater.node));
      continue;
    }

    const childElem = el.childNodes[i + offset];

    if (childUpdater.kind == "remove") {
      childElem.remove();
      offset -= 1;
      continue;
    }

    patch(childElem as HTMLElement, childUpdater);
  }
};
