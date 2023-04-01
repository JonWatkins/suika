import { vDomAttrs, vDomNode } from "./h";
import { isEqual } from "../lib/utils";

type AttrsUpdater = {
  set: vDomAttrs;
  remove: string[];
};

interface SkipOperation {
  kind: "skip";
}

interface UpdateOperation {
  kind: "update";
  attrs: AttrsUpdater;
  childeren: ChildUpdater[];
}

interface ReplaceOperation {
  kind: "replace";
  newNode: vDomNode;
  callback?: (elem: HTMLElement | Text) => void;
}

interface InsertOperation {
  kind: "insert";
  node: vDomNode;
}

interface RemoveOperation {
  kind: "remove";
}

export type vDomUpdater = UpdateOperation | ReplaceOperation | SkipOperation;

export type ChildUpdater =
  | UpdateOperation
  | ReplaceOperation
  | SkipOperation
  | InsertOperation
  | RemoveOperation;

const skip = (): SkipOperation => ({ kind: "skip" });

const replace = (newNode: vDomNode): ReplaceOperation => ({
  kind: "replace",
  newNode,
});

const remove = (): RemoveOperation => ({ kind: "remove" });

const insert = (node: vDomNode): InsertOperation => ({ kind: "insert", node });

const update = (
  attrs: AttrsUpdater,
  childeren: ChildUpdater[]
): UpdateOperation => ({
  kind: "update",
  attrs,
  childeren,
});

export const diff = (
  oldVnode: vDomNode | null,
  newVnode: vDomNode
): vDomUpdater => {
  // If there was no old vNode then we will need to just replace the element
  // with the new vNode
  if (!oldVnode) {
    return replace(newVnode);
  }

  // If both old and new vNode are text and both values are equal we will
  // skip over it
  if (
    oldVnode.kind === "text" &&
    newVnode.kind === "text" &&
    oldVnode.value === newVnode.value
  ) {
    return skip();
  }

  // if both are text vNodes but the values are not equal we will need to
  // replace the element with the new vNode
  if (oldVnode.kind === "text" || newVnode.kind === "text") {
    return replace(newVnode);
  }

  // If both vNodes are components and the oldVnode has an instance we need
  // to copy the instance to the new node an call setProps to trigger a
  // diff on the child elements
  if (
    oldVnode.kind === "component" &&
    newVnode.kind === "component" &&
    oldVnode.component === newVnode.component &&
    oldVnode.instance
  ) {
    newVnode.instance = oldVnode.instance;
    if (isEqual(oldVnode.attrs, newVnode.attrs)) return skip();
    return newVnode.instance._setProps(newVnode.attrs);
  }

  // if the newVnode is a component and not equal to the oldVnode we will need
  // to create a new instance and mount it
  if (newVnode.kind === "component") {
    newVnode.instance = new newVnode.component();
    return {
      kind: "replace",
      newNode: newVnode.instance._initProps(newVnode.attrs),
      callback: (e) => newVnode.instance?._notifyMounted(e),
    };
  }

  // If the oldVnode and newVnode are both functions and the functions are the
  // same we need to return a new diff passing in the oldVnode, and the return
  // value of the newVnode function
  if (
    oldVnode.kind === "function" &&
    newVnode.kind === "function" &&
    oldVnode.component === newVnode.component
  ) {
    return diff(oldVnode, newVnode.component(newVnode.attrs));
  }

  // if the tag of a node has changed we have to replace it completely
  if (oldVnode.tag !== newVnode.tag) {
    return replace(newVnode);
  }

  // Now we can apply the updates to the attributes
  const attrs: AttrsUpdater = {
    remove: Object.keys(oldVnode.attrs).filter(
      (attr) => Object.keys(newVnode).indexOf(attr) === -1
    ),
    set: Object.keys(newVnode.attrs)
      .filter((attr) => oldVnode.attrs[attr] !== newVnode.attrs[attr])
      .reduce(
        (updated, attr) => ({ ...updated, [attr]: newVnode.attrs[attr] }),
        {}
      ),
  };

  const childUpdater: ChildUpdater[] = childsDiff(
    oldVnode.children,
    newVnode.children
  );

  return update(attrs, childUpdater);
};

const childsDiff = (
  oldChildren: vDomNode[],
  newChildren: vDomNode[]
): ChildUpdater[] => {
  const remainingOldChildren: [string | undefined | number, vDomNode][] =
    oldChildren.map((c) => [c.key, c]);

  const remainingNewChildren: [string | undefined | number, vDomNode][] =
    newChildren.map((c) => [c.key, c]);

  const operations: ChildUpdater[] = [];

  let [nextUpdateKey] = remainingOldChildren.find(
    (k) => remainingNewChildren.map((k) => k[0]).indexOf(k[0]) != -1
  ) || [null];

  while (nextUpdateKey) {
    // first remove all old childs before the update
    removeUntilkey(operations, remainingOldChildren, nextUpdateKey);

    // then insert all new childs before the update
    insertUntilKey(operations, remainingNewChildren, nextUpdateKey);

    // create the update
    operations.push(
      diff(remainingOldChildren.shift()[1], remainingNewChildren.shift()[1])
    );

    // find the next update
    [nextUpdateKey] = remainingOldChildren.find(
      (k) => remainingNewChildren.map((k) => k[0]).indexOf(k[0]) != -1
    ) || [null];
  }

  // remove all remaing old childs after the last update
  removeUntilkey(operations, remainingOldChildren, undefined);

  // insert all remaing new childs after the last update
  insertUntilKey(operations, remainingNewChildren, undefined);

  return operations;
};

const removeUntilkey = (
  operations: ChildUpdater[],
  elems: [string | undefined | number, vDomNode][],
  key: string | number | undefined
) => {
  while (elems[0] && elems[0][0] != key) {
    if (elems[0][1].kind == "component") {
      elems[0][1].instance._unmount();
      elems[0][1].instance = null;
    }
    operations.push(remove());
    elems.shift();
  }
};

const insertUntilKey = (
  operations: ChildUpdater[],
  elems: [string | undefined | number, vDomNode][],
  key: string | number | undefined
) => {
  while (elems[0] && elems[0][0] != key) {
    operations.push(insert(elems.shift()[1]));
  }
};
