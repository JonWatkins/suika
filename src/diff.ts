import { render } from "./render";
import { isArray, isDef, isEqual } from "./utils";

export function diff(oldVTree, newVTree) {
  if (!oldVTree) {
    return (node) => {
      const newNode = render(newVTree);
      node.replaceWith(newNode);
      return newNode;
    };
  }

  if (!newVTree) {
    return (node) => {
      if (isArray(oldVTree.children)) {
        unmountChildNodes(oldVTree.children, []);
      }
      node.remove();
      return undefined;
    };
  }

  if (oldVTree.kind === "text" || newVTree.kind === "text") {
    if (oldVTree.value !== newVTree.value) {
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
    if (isEqual(oldVTree.attrs, newVTree.attrs)) return (node) => node;
    newVTree.instance._setAttrs(newVTree.attrs);
    return newVTree.instance._getDiff();
  }

  if (newVTree.kind === "component") {
    newVTree.instance = new newVTree.component();
    newVTree.instance._initState();
    newVTree.instance._initVnode(newVTree.attrs);
    return (node) => {
      const newNode = render(newVTree);
      node.replaceWith(newNode);
      newVTree.instance._notifyMounted(node);
      return newNode;
    };
  }

  if (oldVTree.tag !== newVTree.tag) {
    return (node) => {
      const newNode = render(newVTree);
      if (isArray(oldVTree.children)) {
        unmountChildNodes(oldVTree.children, newVTree.children);
      }
      node.replaceWith(newNode);
      return newNode;
    };
  }

  const patchAttrs = diffAttrs(oldVTree.attrs, newVTree.attrs);

  const patchChildNodes = diffChildNodes(oldVTree.children, newVTree.children);

  return (node) => {
    patchAttrs(node);
    patchChildNodes(node);
    return node;
  };
}

export function diffAttrs(oldAttrs, newAttrs) {
  const attrs = {
    remove: Object.keys(oldAttrs || {}).filter(
      (attr) => !isDef(newAttrs[attr])
    ),
    set: Object.keys(newAttrs || {})
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
      (node as any)[attr] = attrs.set[attr];
    }
  };
}

export function diffChildNodes(oldChildNodes, newChildNodes) {
  const childNodePatches: Array<object> = [];
  const additionalPatches: Array<object> = [];

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
    if (parent) {
      for (const [patch, childNode] of zip(
        childNodePatches,
        parent.childNodes
      )) {
        patch(childNode);
      }

      for (const patch of additionalPatches) {
        patch(parent);
      }
    }

    return parent;
  };
}

export function unmountChildNodes(oldChildNodes, newChildNodes) {
  let i = oldChildNodes.length;
  while (i--) {
    const oldNode = oldChildNodes[i];
    if (oldNode.kind === "component") {
      const newNode = findNewChildComponent(oldNode, newChildNodes);
      if (newNode) {
        unmountChildNodes(
          oldNode.instance._vNode.children,
          newNode.instance._vNode.children
        );
      }
      if (!newNode) {
        oldNode.instance._unmount();
      }
    }
  }
}

export function findNewChildComponent(oldNode, newChildNodes) {
  return newChildNodes.find(
    (newNode) => oldNode.component === newNode.component
  );
}

export const zip = (xs, ys) => {
  const zipped = [];
  for (let i = 0; i < Math.min(xs.length, ys.length); i++) {
    zipped.push([xs[i], ys[i]]);
  }
  return zipped;
};
