import { render } from "./render";
import { isDef, isUndef, isEqual } from "./utils";
export const diff = (oldVTree, newVTree) => {
  if (!oldVTree) {
    return (node) => {
      if (newVTree) {
        const newNode = render(newVTree);
        node.replaceWith(newNode);
        return newNode;
      } else {
        return node;
      }
    };
  }
  if (!newVTree) {
    return (node) => {
      if (oldVTree.kind !== "text") {
        unmountChildNodes(oldVTree, null);
      }
      node.remove();
      return null;
    };
  }
  if (oldVTree.kind === "text" || newVTree.kind === "text") {
    const { value: valueA } = oldVTree;
    const { value: valueB } = newVTree;
    if (valueA !== valueB) {
      return (node) => {
        const newNode = render(newVTree);
        node.replaceWith(newNode);
        return newNode;
      };
    } else {
      return (node) => node;
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
      return (node) => node;
    }
    newVTree.instance._setAttrs(newVTree.attrs);
    return newVTree.instance._getDiff();
  }
  if (newVTree.kind === "component") {
    const instance = new newVTree.component();
    newVTree.instance;
    instance._initState();
    instance._initVnode(newVTree.attrs);
    return (node) => {
      const newNode = render(newVTree);
      node.replaceWith(newNode);
      instance._notifyMounted(node);
      return newNode;
    };
  }
  const { tag: oldTag, children: oldChildNodes, attrs: oldAttrs } = oldVTree;
  const { tag: newTag, children: newChildNodes, attrs: newAttrs } = newVTree;
  if (oldTag !== newTag) {
    return (node) => {
      const newNode = render(newVTree);
      unmountChildNodes(oldVTree, newVTree);
      node.replaceWith(newNode);
      return newNode;
    };
  }
  const patchAttrs = diffAttrs(oldAttrs, newAttrs);
  const patchChildNodes = diffChildNodes(oldChildNodes, newChildNodes);
  return (node) => {
    patchAttrs(node);
    patchChildNodes(node);
    return node;
  };
};
export const diffAttrs = (oldAttrs, newAttrs) => {
  const attrs = {
    remove: Object.keys(oldAttrs).filter((attr) => isUndef(newAttrs[attr])),
    set: Object.keys(newAttrs)
      .filter(
        (attr) => oldAttrs[attr] !== newAttrs[attr] && isDef(newAttrs[attr])
      )
      .reduce((updated, attr) => ({ ...updated, [attr]: newAttrs[attr] }), {}),
  };
  return (node) => {
    for (const attr of attrs.remove) {
      node.removeAttribute(attr);
    }
    for (const attr in attrs.set) {
      node[attr] = attrs.set[attr];
    }
  };
};
export const diffChildNodes = (oldChildNodes, newChildNodes) => {
  const childNodePatches = [];
  const additionalPatches = [];
  oldChildNodes.forEach((oldChild, i) => {
    childNodePatches.push(diff(oldChild, newChildNodes[i]));
  });
  for (const additionalChild of newChildNodes.slice(oldChildNodes.length)) {
    additionalPatches.push((node) => {
      node.appendChild(render(additionalChild));
      return node;
    });
  }
  return (parent) => {
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
export const unmountChildNodes = (oldTree, newTree, toUnmount = []) => {
  // TODO: implment unmount hooks
};
export const zip = (xs, ys) => {
  const zipped = [];
  for (let i = 0; i < Math.min(xs.length, ys.length); i++) {
    zipped.push([xs[i], ys[i]]);
  }
  return zipped;
};
//# sourceMappingURL=diff.js.map
