// @ts-nocheck

import { createElement } from "suika";

export const NotFound = ({ currentPath }) => {
  return createElement(
    "h1",
    {},
    "404 Not Found",
    createElement("p", {}, `Path not found ${currentPath}`),
  );
};
