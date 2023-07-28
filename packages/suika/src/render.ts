// @ts-nocheck
import { workLoop } from "./scheduler";

import {
  setDeletions,
  setNextUnitOfWork,
  setWipRoot,
  getCurrentRoot,
} from "./globals";

export const render = (element, container) => {
  const wipRoot = {
    dom: container,
    props: {
      children: [element],
    },
    alternate: getCurrentRoot(),
  };

  setWipRoot(wipRoot);
  setDeletions([]);
  setNextUnitOfWork(wipRoot);
  requestIdleCallback(workLoop);
};
