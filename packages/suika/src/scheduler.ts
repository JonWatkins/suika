// @ts-nocheck

import { UPDATE_TAG, PLACEMENT_TAG, DELETION_TAG } from "./globals";
import { runEffects, cancelEffects } from "./hooks";
import { updateDom, createDom } from "./vdom";

import {
  getNextUnitOfWork,
  setNextUnitOfWork,
  getWipRoot,
  setWipRoot,
  setCurrentRoot,
  getDeletions,
  setWipFiber,
  setHookIndex,
} from "./globals";

export const commitRoot = () => {
  const wipRoot = getWipRoot();
  const deletions = getDeletions();

  deletions.forEach(commitWork);
  commitWork(wipRoot.child);

  setCurrentRoot(wipRoot);
  setWipRoot(null);
};

export const commitWork = (fiber) => {
  if (!fiber) {
    return;
  }

  let domParentFiber = fiber.parent;

  while (!domParentFiber.dom) {
    domParentFiber = domParentFiber.parent;
  }

  const domParent = domParentFiber.dom;

  if (fiber.effectTag === PLACEMENT_TAG) {
    if (fiber.dom != null) {
      domParent.appendChild(fiber.dom);
    }
    runEffects(fiber);
  } else if (fiber.effectTag === UPDATE_TAG) {
    cancelEffects(fiber);
    if (fiber.dom != null) {
      updateDom(fiber.dom, fiber.alternate.props, fiber.props);
    }
    runEffects(fiber);
  } else if (fiber.effectTag === DELETION_TAG) {
    cancelEffects(fiber);
    commitDeletion(fiber, domParent);
    return;
  }

  commitWork(fiber.child);
  commitWork(fiber.sibling);
};

export const commitDeletion = (fiber, domParent) => {
  if (fiber.dom) {
    domParent.removeChild(fiber.dom);
  } else {
    commitDeletion(fiber.child, domParent);
  }
};

export const updateFunctionComponent = (fiber) => {
  const wipFiber = fiber;
  wipFiber.hooks = [];
  setWipFiber(wipFiber);
  setHookIndex(0);

  const children = [fiber.type(fiber.props)];
  reconcileChildren(fiber, children);
};

export const updateHostComponent = (fiber) => {
  if (!fiber.dom) {
    fiber.dom = createDom(fiber);
  }

  reconcileChildren(fiber, fiber.props.children);
};

export const reconcileChildren = (wipFiber, elements) => {
  const deletions = getDeletions();
  let index = 0;
  let oldFiber = wipFiber.alternate && wipFiber.alternate.child;
  let prevSibling = null;

  while (index < elements.length || oldFiber) {
    const element = elements[index];
    let newFiber = null;

    const sameType = oldFiber && element && element.type == oldFiber.type;

    if (sameType) {
      newFiber = {
        type: oldFiber.type,
        props: element.props,
        dom: oldFiber.dom,
        parent: wipFiber,
        alternate: oldFiber,
        effectTag: UPDATE_TAG,
      };
    }

    if (element && !sameType) {
      newFiber = {
        type: element.type,
        props: element.props,
        dom: null,
        parent: wipFiber,
        alternate: null,
        effectTag: PLACEMENT_TAG,
      };
    }

    if (oldFiber && !sameType) {
      oldFiber.effectTag = DELETION_TAG;
      deletions.push(oldFiber);
    }

    if (oldFiber) {
      oldFiber = oldFiber.sibling;
    }

    if (index === 0) {
      wipFiber.child = newFiber;
    } else if (element) {
      prevSibling.sibling = newFiber;
    }

    prevSibling = newFiber;
    index++;
  }
};

export const performUnitOfWork = (fiber) => {
  const isFunctionComponent = fiber.type instanceof Function;

  if (isFunctionComponent) {
    updateFunctionComponent(fiber);
  } else {
    updateHostComponent(fiber);
  }

  if (fiber.child) {
    return fiber.child;
  }

  let nextFiber = fiber;

  while (nextFiber) {
    if (nextFiber.sibling) {
      return nextFiber.sibling;
    }

    nextFiber = nextFiber.parent;
  }
};

export const workLoop = () => {
  let shouldYield = false;
  let nextUnitOfWork = getNextUnitOfWork();
  const wipRoot = getWipRoot();

  while (nextUnitOfWork && !shouldYield) {
    nextUnitOfWork = performUnitOfWork(nextUnitOfWork);
    if (nextUnitOfWork) shouldYield = true;
  }

  setNextUnitOfWork(nextUnitOfWork);

  if (!nextUnitOfWork && wipRoot) {
    commitRoot();
  }

  setTimeout(workLoop);
};
