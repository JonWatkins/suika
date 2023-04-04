import { render } from "./render";
import { isDef, isUndef, isEqual } from "./utils";
import { vNode, vAttrs, vText, vElement } from "./vdom";

export type AttrsUpdater = {
  set: vAttrs;
  remove: string[];
};

export function diff(oldVTree: vNode | null, newVTree: vNode | null): Function {
  if (!oldVTree) {
    return (node: HTMLElement): HTMLElement => {
      if (newVTree) {
        const newNode = render(newVTree) as HTMLElement;
        node.replaceWith(newNode);
        return newNode;
      } else {
        return node;
      }
    };
  }

  if (!newVTree) {
    return (node: HTMLElement): null => {
      if (oldVTree.kind !== "text") {
        unmountChildNodes(oldVTree, null);
      }

      node.remove();
      return null;
    };
  }

  if (oldVTree.kind === "text" || newVTree.kind === "text") {
    const { value: valueA } = oldVTree as vText;
    const { value: valueB } = newVTree as vText;

    if (valueA !== valueB) {
      return (node: Text): Text => {
        const newNode = render(newVTree) as Text;
        node.replaceWith(newNode);
        return newNode;
      };
    } else {
      return (node: Text): Text => node;
    }
  }

  if (oldVTree.kind === "function" && newVTree.kind === "function") {
    return diff(
      oldVTree,
      newVTree.component(newVTree.attrs, newVTree.children)
    );
  }

  if (
    oldVTree.kind === "component" &&
    newVTree.kind === "component" &&
    oldVTree.component === newVTree.component &&
    oldVTree.instance
  ) {
    newVTree.instance = oldVTree.instance;

    if (isEqual(oldVTree.attrs, newVTree.attrs)) {
      return (node: HTMLElement): HTMLElement => node;
    }
    newVTree.instance._setAttrs(newVTree.attrs);

    return newVTree.instance._getDiff();
  }

  if (newVTree.kind === "component") {
    const instance = new newVTree.component();
    newVTree.instance;
    instance._initState();
    instance._initVnode(newVTree.attrs);

    return (node: HTMLElement): HTMLElement => {
      const newNode = render(newVTree) as HTMLElement;
      node.replaceWith(newNode);
      instance._notifyMounted(node);
      return newNode;
    };
  }

  const {
    tag: oldTag,
    children: oldChildNodes,
    attrs: oldAttrs,
  } = oldVTree as vElement;

  const {
    tag: newTag,
    children: newChildNodes,
    attrs: newAttrs,
  } = newVTree as vElement;

  if (oldTag !== newTag) {
    return (node: HTMLElement): HTMLElement => {
      const newNode = render(newVTree) as HTMLElement;
      unmountChildNodes(oldVTree, newVTree);
      node.replaceWith(newNode);
      return newNode;
    };
  }

  const patchAttrs = diffAttrs(oldAttrs, newAttrs);
  const patchChildNodes = diffChildNodes(oldChildNodes, newChildNodes);

  return (node: HTMLElement): HTMLElement => {
    patchAttrs(node);
    patchChildNodes(node);
    return node;
  };
}

export function diffAttrs(oldAttrs: vAttrs, newAttrs: vAttrs) {
  const attrs: AttrsUpdater = {
    remove: Object.keys(oldAttrs).filter((attr) => isUndef(newAttrs[attr])),
    set: Object.keys(newAttrs)
      .filter(
        (attr) => oldAttrs[attr] !== newAttrs[attr] && isDef(newAttrs[attr])
      )
      .reduce((updated, attr) => ({ ...updated, [attr]: newAttrs[attr] }), {}),
  };

  return (node: HTMLElement) => {
    for (const attr of attrs.remove) {
      node.removeAttribute(attr);
    }

    for (const attr in attrs.set) {
      (node as any)[attr] = attrs.set[attr];
    }
  };
}

export function diffChildNodes(oldChildNodes: vNode[], newChildNodes: vNode[]) {
  const childNodePatches: Array<Function> = [];
  const additionalPatches: Array<Function> = [];

  oldChildNodes.forEach((oldChild: vNode, i: number) => {
    childNodePatches.push(diff(oldChild, newChildNodes[i]));
  });

  for (const additionalChild of newChildNodes.slice(oldChildNodes.length)) {
    additionalPatches.push((node: HTMLElement): HTMLElement => {
      node.appendChild(render(additionalChild));
      return node;
    });
  }

  return (parent: HTMLElement) => {
    for (const [patch, childNode] of zip(
      childNodePatches,
      Array.from(parent.childNodes)
    )) {
      patch(childNode);
    }

    for (const patch of additionalPatches) {
      patch(parent);
    }

    return parent;
  };
}

export function unmountChildNodes(oldNode: vNode, newNode: vNode | null) {
  //console.log(oldNode, newNode);
}

export const zip = (xs: Array<any>, ys: Array<any>) => {
  const zipped = [];
  for (let i = 0; i < Math.min(xs.length, ys.length); i++) {
    zipped.push([xs[i], ys[i]]);
  }
  return zipped;
};
