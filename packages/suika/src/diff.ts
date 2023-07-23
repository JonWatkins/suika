import { render, dangerouslySetHtmlContent } from "./render";
import { isDef, isUndef, isEqual, isObject, zip } from "./utils";
import { vNode, vAttrs, vText, vElement, vFunction } from "./vdom";

export type AttrsUpdater = {
  set: vAttrs;
  remove: string[];
};

export const diff = (
  oldVTree: vNode | null,
  newVTree: vNode | null
): Function => {
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
        unmountChildNodes(oldVTree, undefined);
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
    newVTree.instance = instance;
    instance._initState();
    instance._initVnode(newVTree.attrs);

    return (node: HTMLElement): HTMLElement => {
      unmountChildNodes(oldVTree, newVTree);
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
      unmountChildNodes(oldVTree, newVTree);
      const newNode = render(newVTree) as HTMLElement;
      node.replaceWith(newNode);
      return newNode;
    };
  }

  const patchAttrs = diffAttrs(oldAttrs, newAttrs);

  const patchChildNodes = isObject(newAttrs.dangerouslySetHtml)
    ? diffDangerouslySetHtml(oldAttrs, newAttrs)
    : diffChildNodes(oldChildNodes, newChildNodes);

  return (node: HTMLElement): HTMLElement => {
    patchAttrs(node);
    patchChildNodes(node);
    return node;
  };
};

export const diffAttrs = (oldAttrs: vAttrs, newAttrs: vAttrs) => {
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
};

export const diffChildNodes = (
  oldChildNodes: vNode[],
  newChildNodes: vNode[]
) => {
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
};

export const diffDangerouslySetHtml = (
  oldAttrs: vAttrs,
  newAttrs: vAttrs
): Function => {
  return (parent: HTMLElement) => {
    if (oldAttrs.dangerouslySetHtml !== newAttrs.dangerouslySetHtml) {
      return dangerouslySetHtmlContent(parent, newAttrs);
    }

    return parent;
  };
};

export const unmountChildNodes = (
  oldTree: vNode,
  newTree: vNode | undefined
) => {
  const toUnmount = getToUnmount(oldTree, newTree);
  let i = toUnmount.length;

  while (i--) {
    toUnmount[i]._unmount();
  }
};

export const getToUnmount = (
  oldTree: vNode,
  newTree: vNode | undefined,
  toUnmount: Array<any> = []
) => {
  if (oldTree.kind === "element" || oldTree.kind === "function") {
    const oldChildNodes = oldTree.children;

    if (oldTree.children.length) {
      for (const [index, childNode] of Array.from(oldChildNodes.entries())) {
        const newChildNode = getNewChildNode(index, newTree);
        getToUnmount(childNode, newChildNode, toUnmount);
      }
    }
  }

  if (oldTree.kind === "component") {
    const childNodes = oldTree.instance && oldTree.instance._vNode;

    if (!newTree) {
      toUnmount.push(oldTree.instance);
      getToUnmount(childNodes as vNode, undefined, toUnmount);
    } else if (
      newTree.kind === "component" &&
      oldTree.component !== newTree.component
    ) {
      const newChildNodes = newTree.instance && newTree.instance._vNode;
      toUnmount.push(oldTree.instance);
      getToUnmount(childNodes as vNode, newChildNodes as vNode, toUnmount);
    }
  }

  return toUnmount;
};

export const getNewChildNode = (
  index: number,
  newTree: vNode | undefined
): vNode | undefined => {
  if (newTree && (newTree.kind === "element" || newTree.kind === "function")) {
    return (newTree as vElement | vFunction).children[index];
  }
  return undefined;
};
