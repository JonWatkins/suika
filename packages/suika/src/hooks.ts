// @ts-nocheck

import {
  getHookIndex,
  setHookIndex,
  getCurrentRoot,
  getWipFiber,
  setWipRoot,
  setNextUnitOfWork,
  setDeletions,
} from "./globals";

export const useState = (initial) => {
  const wipFiber = getWipFiber();
  let hookIndex = getHookIndex();

  const oldHook =
    wipFiber.alternate &&
    wipFiber.alternate.hooks &&
    wipFiber.alternate.hooks[hookIndex];

  const hook = {
    tag: "state",
    state: oldHook ? oldHook.state : initial,
    queue: [],
  };

  const actions = oldHook ? oldHook.queue : [];

  actions.forEach((action) => {
    hook.state = typeof actions === "function" ? action(hook.state) : action;
  });

  const setState = (action) => {
    const currentRoot = getCurrentRoot();
    hook.queue.push(action);

    const wipRoot = {
      dom: currentRoot.dom,
      props: currentRoot.props,
      alternate: currentRoot,
    };

    setWipRoot(wipRoot);
    setNextUnitOfWork(wipRoot);
    setDeletions([]);
  };

  wipFiber.hooks.push(hook);
  setHookIndex(hookIndex++);

  return [hook.state, setState];
};

const hasDepsChanged = (prevDeps, nextDeps) =>
  !prevDeps ||
  !nextDeps ||
  prevDeps.length !== nextDeps.length ||
  prevDeps.some((dep, index) => dep !== nextDeps[index]);

export const useEffect = (effect, deps) => {
  const wipFiber = getWipFiber();
  let hookIndex = getHookIndex();

  const oldHook =
    wipFiber.alternate &&
    wipFiber.alternate.hooks &&
    wipFiber.alternate.hooks[hookIndex];

  const hasChanged = hasDepsChanged(oldHook ? oldHook.deps : undefined, deps);

  const hook = {
    tag: "effect",
    effect: hasChanged ? effect : null,
    cancel: hasChanged && oldHook && oldHook.cancel,
    deps,
  };

  wipFiber.hooks.push(hook);
  setHookIndex(hookIndex++);
};

export const cancelEffects = (fiber) => {
  if (fiber.hooks) {
    fiber.hooks
      .filter((hook) => hook.tag === "effect" && hook.cancel)
      .forEach((effectHook) => {
        effectHook.cancel();
      });
  }
};

export const runEffects = (fiber) => {
  if (fiber.hooks) {
    fiber.hooks
      .filter((hook) => hook.tag === "effect" && hook.effect)
      .forEach((effectHook) => {
        effectHook.cancel = effectHook.effect();
      });
  }
};
